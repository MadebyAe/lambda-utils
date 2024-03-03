use lambda_http::Request;
use serde_json::to_string;
use std::collections::HashMap;

pub fn get_header_value(request: &Request, value: String) -> String {
    return request.headers().get(value).map_or_else(
        || "Unknown header value".to_string(),
        |header_value| {
            header_value
                .to_str()
                .unwrap_or("Invalid header value")
                .to_string()
        },
    );
}

pub fn get_header_user_agent(request: &Request) -> String {
    return get_header_value(request, "User-Agent".to_string());
}

pub fn get_header_cookies(request: &Request) -> String {
    let mut cookies = HashMap::new();

    if let Some(cookie_header) = request.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            cookie_str
                .split(';')
                .filter_map(|cookie| {
                    let parts: Vec<&str> = cookie.splitn(2, '=').collect();

                    if parts.len() == 2 {
                        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                    } else {
                        None
                    }
                })
                .for_each(|(name, value)| {
                    cookies.insert(name, value);
                });
        }
    }

    return to_string(&cookies).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::header::{HeaderMap, HeaderValue};
    use lambda_http::{Body, Request};

    fn mock_request(headers: Vec<(&str, &str)>) -> Request {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(key, HeaderValue::from_static(value));
        }

        Request::new(Body::Empty).with_headers(header_map)
    }

    #[test]
    fn test_get_header_value_known() {
        let request = mock_request(vec![("User-Agent", "TestAgent")]);
        assert_eq!(
            get_header_value(&request, "User-Agent".to_string()),
            "TestAgent"
        );
    }

    #[test]
    fn test_get_header_value_unknown() {
        let request = mock_request(vec![]);
        assert_eq!(
            get_header_value(&request, "User-Agent".to_string()),
            "Unknown header value"
        );
    }

    #[test]
    fn test_get_header_user_agent() {
        let request = mock_request(vec![("User-Agent", "TestAgent")]);
        assert_eq!(get_header_user_agent(&request), "TestAgent");
    }

    #[test]
    fn test_get_header_cookies() {
        let request = mock_request(vec![("Cookie", "username=user; session=token")]);
        assert_eq!(
            get_header_cookies(&request),
            r#"{"session":"token","username":"user"}"#
        );
    }
}
