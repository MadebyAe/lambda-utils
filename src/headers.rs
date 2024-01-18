use lambda_http::{Request};
use std::collections::{HashMap};
use serde_json::{to_string};

pub fn get_header_user_agent(request: &Request) -> String {
    return request
        .headers()
        .get("User-Agent")
        .map_or_else(
            || "Unknown User-Agent".to_string(),
            |header_value| header_value.to_str().unwrap_or("Invalid User-Agent").to_string(),
        );
}

pub fn get_header_cookies(request: &Request) -> String {
    let mut cookies = HashMap::new();

    if let Some(cookie_header) = request.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            cookie_str.split(';').filter_map(|cookie| {
                let parts: Vec<&str> = cookie.splitn(2, '=').collect();

                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            }).for_each(|(name, value)| {
                cookies.insert(name, value);
            });
        }
    }

    return to_string(&cookies).unwrap();
}
