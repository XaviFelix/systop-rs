use std::collections::HashMap;

//NOTE: Used for data with whitespaces like the cache key
pub fn parse_u32(map: &HashMap<&str, &str>, key: &str) -> Option<u32> {
    map.get(key)
        .and_then(|data| data.split_whitespace().next())
        .and_then(|s| s.parse::<u32>().ok())
}

//TODO: make better names
pub fn parse_u32_v2(map: &HashMap<&str, &str>, key: &str) -> Option<u32> {
    map.get(key)?.parse::<u32>().ok()
}

pub fn parse_f32(map: &HashMap<&str, &str>, key: &str) -> Option<f32> {
    map.get(key)?.parse::<f32>().ok()
}

pub fn parse_string(map: &HashMap<&str, &str>, key: &str) -> Option<String> {
    map.get(key).map(|s| s.to_string())
}
