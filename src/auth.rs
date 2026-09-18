use lambda_http::request::RequestContext;
use lambda_http::{Request, RequestExt};

/// Reads the `custom:org_id` claim from the caller's Cognito ID token, as
/// delivered by API Gateway's HTTP API native JWT authorizer
/// (`requestContext.authorizer.jwt.claims`). `None` covers every case where
/// the caller isn't properly scoped to an org - unauthenticated, an access
/// token instead of an ID token (access tokens don't carry custom
/// attributes), or a user with no `custom:org_id` set - callers must treat
/// `None` as a hard reject, never as "no restriction."
pub fn caller_org_id(request: &Request) -> Option<String> {
    match request.request_context_ref() {
        Some(RequestContext::ApiGatewayV2(context)) => context
            .authorizer
            .as_ref()
            .and_then(|authorizer| authorizer.jwt.as_ref())
            .and_then(|jwt| jwt.claims.get("custom:org_id").cloned()),
        _ => None,
    }
}

/// `require_org_id!(request)` - the `caller_org_id` check every org-scoped
/// handler needs, without repeating the `let Some(...) = ... else { return
/// ...; }` block at every call site. A macro, not a function: it has to
/// `return` out of the *caller's* function (an `async fn` returning
/// `Result<Response<Body>, Error>`), which a plain function can't do.
/// Needs the `response` feature too (pulled in automatically - `auth`
/// depends on it in Cargo.toml) for `json_error!`.
#[macro_export]
macro_rules! require_org_id {
    ($request:expr) => {
        match $crate::auth::caller_org_id(&$request) {
            Some(org_id) => org_id,
            None => {
                return Ok($crate::json_error!(
                    lambda_http::http::StatusCode::UNAUTHORIZED,
                    "missing or invalid org claim"
                ))
            }
        }
    };
}
pub use require_org_id;

#[cfg(test)]
mod auth_tests {
    use super::*;
    use aws_lambda_events::apigw::{
        ApiGatewayRequestAuthorizer, ApiGatewayRequestAuthorizerJwtDescription,
        ApiGatewayV2httpRequestContext,
    };
    use lambda_http::{Body, Request, Response};
    use std::collections::HashMap;

    fn request_with_claims(claims: Option<HashMap<String, String>>) -> Request {
        let authorizer = claims.map(|claims| ApiGatewayRequestAuthorizer {
            jwt: Some(ApiGatewayRequestAuthorizerJwtDescription {
                claims,
                scopes: None,
            }),
            ..Default::default()
        });

        Request::new(Body::Empty).with_request_context(RequestContext::ApiGatewayV2(
            ApiGatewayV2httpRequestContext {
                authorizer,
                ..Default::default()
            },
        ))
    }

    #[test]
    fn returns_the_org_id_claim_when_present() {
        let claims = HashMap::from([("custom:org_id".to_string(), "org_pepsi".to_string())]);
        let request = request_with_claims(Some(claims));

        assert_eq!(caller_org_id(&request), Some("org_pepsi".to_string()));
    }

    #[test]
    fn returns_none_when_unauthenticated() {
        let request = Request::new(Body::Empty);

        assert_eq!(caller_org_id(&request), None);
    }

    #[test]
    fn returns_none_when_claims_exist_but_org_id_is_absent() {
        // The access-token case: authenticated, but no custom attributes -
        // must not be confused with "no restriction".
        let claims = HashMap::from([("sub".to_string(), "user-123".to_string())]);
        let request = request_with_claims(Some(claims));

        assert_eq!(caller_org_id(&request), None);
    }

    #[test]
    fn returns_none_when_authorizer_has_no_jwt_claims_at_all() {
        let request = request_with_claims(None);

        assert_eq!(caller_org_id(&request), None);
    }

    // Mirrors real handler shape exactly (`request: Request`, returns
    // `Result<Response<Body>, lambda_http::Error>`) since the macro's
    // early-return only type-checks against that specific shape.
    fn handler_using_macro(request: Request) -> Result<Response<Body>, lambda_http::Error> {
        let org_id = require_org_id!(request);
        Ok(Response::builder()
            .status(200)
            .body(Body::from(org_id))
            .unwrap())
    }

    #[test]
    fn require_org_id_returns_the_org_id_when_present() {
        let claims = HashMap::from([("custom:org_id".to_string(), "org_pepsi".to_string())]);
        let request = request_with_claims(Some(claims));

        let response = handler_using_macro(request).unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!(response.into_body(), Body::from("org_pepsi"));
    }

    #[test]
    fn require_org_id_returns_401_when_missing() {
        let request = Request::new(Body::Empty);

        let response = handler_using_macro(request).unwrap();

        assert_eq!(
            response.status(),
            lambda_http::http::StatusCode::UNAUTHORIZED
        );
    }
}
