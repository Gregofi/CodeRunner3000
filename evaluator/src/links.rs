use rand::{distributions::Alphanumeric, Rng};

use redis::Commands;
use anyhow::Result;
use lazy_static::lazy_static;

lazy_static!{
    static ref LINKS_REDIS_HOST: String = std::env::var("LINKS_REDIS_HOST").unwrap();
}

fn generate_link_id() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
    s
}

fn _generate_link(redis: &mut redis::Connection, payload: String, tries: i32) -> Result<String> {
    if tries > 10 {
        return Err(anyhow::anyhow!("Failed to generate link"));
    }
    let link_id = generate_link_id();
    let exists: Option<String> = redis.get(&link_id)?;
    if exists.is_some() {
        return _generate_link(redis, payload, tries + 1);
    }
    redis.set(&link_id, payload)?;
    Ok(link_id)
}

pub fn generate_link(payload: String) -> Result<String> {
    // TODO: In case this ever proves to be a bottlenect, use multiplexed
    // connection and use it across jobs.
    let client = redis::Client::open(format!("redis://{}/", *LINKS_REDIS_HOST))?;
    let mut con = client.get_connection()?;
    _generate_link(&mut con, payload, 0)
}
