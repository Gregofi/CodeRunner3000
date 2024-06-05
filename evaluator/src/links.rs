use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{extract, response};
use rand::Rng;
use redis::AsyncCommands;
use serde::Serialize;

use crate::{AppError, AppState};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const PAYLOAD_LIMIT: usize = 1500;
pub const BURST_SIZE: u32 = 20;

fn random_base62(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..ALPHABET.len());
            ALPHABET.chars().nth(idx).unwrap()
        })
        .collect()
}

/// The data stored in redis. Small keys on purpose to avoid storing large payloads.
#[derive(serde::Serialize, serde::Deserialize)]
struct SavedData {
    /// payload
    p: String,
    /// crc32
    crc: u32,
    /// version
    v: u32,
}

#[derive(serde::Serialize)]
pub struct GenerateResponse {
    pub key: String,
}

async fn save_string_to_redis(mut redis: redis::aio::ConnectionManager, key: String, value: String) -> Result<()> {
    let crc = crc32fast::hash(&value.as_bytes());
    let saved_data = SavedData {
        p: value,
        crc,
        v: 1,
    };
    let _: () = redis.set(&key, serde_json::to_string(&saved_data)?).await?;
    Ok(())
}

async fn read_string_from_redis(mut redis: redis::aio::ConnectionManager, key: String) -> Result<Option<String>> {
    let data: Option<String> = redis.get(&key).await?;
    match data {
        Some(data) => {
            let saved_data: SavedData = serde_json::from_str(&data)?;
            if saved_data.v == 1 {
                let crc = crc32fast::hash(&saved_data.p.as_bytes());
                if crc == saved_data.crc {
                    Ok(Some(saved_data.p))
                } else {
                    Err(anyhow::anyhow!("CRC mismatch"))
                }
            } else {
                Err(anyhow::anyhow!("Unknown version, only version 1 is supported"))
            }
        }
        None => Ok(None),
    }
}

async fn generate_link(
    mut redis: redis::aio::ConnectionManager,
    payload: String,
) -> Result<GenerateResponse> {
    let mut tries = 0;
    while tries < 10 {
        let key = random_base62(12);
        let exists: Option<String> = redis.get(&key).await?;
        if exists.is_none() {
            save_string_to_redis(redis.clone(), key.clone(), payload.clone()).await?;
            return Ok(GenerateResponse { key });
        }
        tries += 1;
    }
    Err(anyhow::anyhow!("Failed to generate a unique link"))
}

pub async fn new_handler(
    extract::State(state): extract::State<Arc<AppState>>,
    payload: String,
) -> Result<response::Json<GenerateResponse>, AppError> {
    if payload.len() > PAYLOAD_LIMIT {
        return Err(AppError {
            err: anyhow::anyhow!("Payload too large"),
            status_code: Some(axum::http::StatusCode::PAYLOAD_TOO_LARGE),
        });
    }

    let resp = generate_link(state.redis.clone(), payload)
        .await
        .context("Failed to generate link")?;
    Ok(response::Json(resp))
}

#[derive(serde::Serialize)]
pub struct GetResponse {
    pub key: String,
    pub value: String,
}

pub async fn get_handler(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(key): extract::Path<String>,
) -> Result<response::Json<GetResponse>, AppError> {
    let payload = read_string_from_redis(state.redis.clone(), key.clone())
        .await
        .context("Failed to read link")?;
    match payload {
        Some(value) => Ok(response::Json(GetResponse { key, value })),
        None => Err(AppError {
            err: anyhow::anyhow!("Link not found"),
            status_code: Some(axum::http::StatusCode::NOT_FOUND),
        }),
    }
}
