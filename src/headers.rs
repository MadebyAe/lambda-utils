use lambda_http::Request;
use serde_json::{json, Value};
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

pub fn get_header_cookies(request: &Request) -> Value {
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

    return json!(&cookies);
}

#[cfg(test)]
mod headers_tests {
    use super::*;
    use lambda_http::http::header::{HeaderName, HeaderValue};
    use lambda_http::{Body, Request};

    fn mock_request(headers: Vec<(&str, &str)>) -> Request {
        let mut request = Request::new(Body::Empty); // Correct use of Request::new with Body::Empty for lambda_http

        for (key, value) in headers {
            match key.parse::<HeaderName>() {
                Ok(parsed_key) => match HeaderValue::from_str(value) {
                    Ok(header_value) => {
                        request.headers_mut().insert(parsed_key, header_value);
                    }
                    Err(e) => eprintln!("Failed to create HeaderValue: {}", e),
                },
                Err(e) => eprintln!("Failed to parse HeaderName: {}", e),
            }
        }

        return request;
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
        let request = mock_request(vec![("Cookie", "username=username; session=session;")]);
        assert_eq!(
            get_header_cookies(&request),
            json!({ "session":"session","username":"username" })
        );
    }
}
