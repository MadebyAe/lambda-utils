use aws_lambda_events::event::apigw::{
    ApiGatewayProxyRequestContext, ApiGatewayV2httpRequestContext,
    ApiGatewayV2httpRequestContextHttpDescription,
};
use lambda_http::request::RequestContext;
use lambda_http::{Request, RequestExt};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub fn get_ip_v4(request: &Request) -> IpAddr {
    let source_ip: String = match request.request_context() {
        RequestContext::ApiGatewayV1(ApiGatewayProxyRequestContext { identity, .. }) => identity
            .source_ip
            .unwrap_or_else(|| Ipv4Addr::UNSPECIFIED.to_string()),
        RequestContext::ApiGatewayV2(ApiGatewayV2httpRequestContext {
            http: ApiGatewayV2httpRequestContextHttpDescription { source_ip, .. },
            ..
        }) => source_ip.unwrap_or_else(|| Ipv4Addr::UNSPECIFIED.to_string()),
        _ => Ipv4Addr::UNSPECIFIED.to_string(),
    };

    return IpAddr::from_str(source_ip.as_str()).unwrap();
}

pub fn get_ip_v6(request: &Request) -> IpAddr {
    let source_ip: String = match request.request_context() {
        RequestContext::ApiGatewayV1(ApiGatewayProxyRequestContext { identity, .. }) => identity
            .source_ip
            .unwrap_or_else(|| Ipv6Addr::UNSPECIFIED.to_string()),
        RequestContext::ApiGatewayV2(ApiGatewayV2httpRequestContext {
            http: ApiGatewayV2httpRequestContextHttpDescription { source_ip, .. },
            ..
        }) => source_ip.unwrap_or_else(|| Ipv6Addr::UNSPECIFIED.to_string()),
        _ => Ipv6Addr::UNSPECIFIED.to_string(),
    };

    return IpAddr::from_str(source_ip.as_str()).unwrap();
}
