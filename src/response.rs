#[macro_export]
macro_rules! json_error {
    ($status_code:expr, $message:expr) => {{
        use chrono::Utc;
        use lambda_http::Body;
        use lambda_http::http::{Response, StatusCode};
        use serde_json::json;

        fn error_code_to_type(status: StatusCode) -> &'static str {
            match status {
                StatusCode::BAD_REQUEST => "BadRequest",
                StatusCode::FORBIDDEN => "Forbidden",
                StatusCode::INTERNAL_SERVER_ERROR => "InternalServerError",
                StatusCode::UNAUTHORIZED => "Unauthorized",
                _ => "Unknown",
            }
        }

        let now = Utc::now();
        let body = json!({
            "code": $status_code.as_u16(),
            "message": $message,
            "timestamp": now.timestamp_millis(),
            "type": error_code_to_type($status_code),
        }).to_string();

        Response::builder()
            .status($status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .expect("Failed to construct error response")
    }};
}
pub use json_error;

#[macro_export]
macro_rules! json_ok {
    ($data:tt) => {{
        use lambda_http::http::Response;
        use lambda_http::Body;
        use serde_json::json;

        let body = json!($data).to_string();

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .expect("Failed to construct success response")
    }};
}
pub use json_ok;

#[cfg(test)]
mod response_tests {
    use super::*;
    use lambda_http::http::StatusCode;
    use lambda_http::{Body, Response};
    use serde_json::{from_slice, Value};

    fn extract_body(response: Response<Body>) -> Value {
        let body = response.into_body();

        return from_slice(&body).unwrap_or_default();
    }

    #[test]
    fn json_error_macro_test() {
        let response = json_error!(StatusCode::FORBIDDEN, "Forbidden access");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = extract_body(response);
        assert_eq!(body["code"], 403);
        assert_eq!(body["message"], "Forbidden access");
        assert_eq!(body["type"], "Forbidden");
    }

    #[test]
    fn json_ok_macro_test() {
        let response = json_ok!({ "data": "Success" });
        assert_eq!(response.status(), StatusCode::OK);

        let body = extract_body(response);
        assert_eq!(body["data"], "Success");
    }
}
