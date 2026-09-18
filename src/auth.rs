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

#[cfg(test)]
mod auth_tests {
    use super::*;
    use aws_lambda_events::apigw::{
        ApiGatewayRequestAuthorizer, ApiGatewayRequestAuthorizerJwtDescription,
        ApiGatewayV2httpRequestContext,
    };
    use lambda_http::{Body, Request};
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
}
