use rand::Rng;
use redis::Commands;
use anyhow::Result;

fn generate_link_id() -> String {
    todo!()
}

fn _generate_link(redis: &mut redis::Connection, payload: String, tries: i32) -> Result<String> {
    if tries > 10 {
        return Err(anyhow::anyhow!("Failed to generate link"));
    }
    let link_id = generate_link_id();
    let exists = redis.get(&link_id);
    if exists.is_ok() {
        return _generate_link(redis, payload, tries + 1);
    }
    redis.set(&link_id, payload);
    Ok(link_id)
}

pub fn generate_link(payload: String) -> Result<String> {
    Ok("".to_string())
}
