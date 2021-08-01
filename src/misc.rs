use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use http::{
    header::{HeaderValue, USER_AGENT},
    StatusCode,
};

#[derive(Debug)]
pub struct ExtractUserAgent(pub HeaderValue);

#[async_trait]
impl<B> FromRequest<B> for ExtractUserAgent
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_agent = req.headers().and_then(|headers| headers.get(USER_AGENT));

        if let Some(user_agent) = user_agent {
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}
