
use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct BodyJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for BodyJson<T>
where
    T: serde::de::DeserializeOwned,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<tower::BoxError>,
{
    type Rejection = JsonRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        use bytes::Buf;

        if has_content_type(req, "application/json")? {
            let body = req.take_body().ok_or(JsonRejection::BodyAlreadyExtracted)?;
            //let body = axum::extract::take_body(req)?;

            let buf = hyper::body::aggregate(body)
                .await
                .map_err(|_| JsonRejection::InvalidJsonBody)?;

            let value = serde_json::from_reader(buf.reader())
                .map_err(|_| JsonRejection::InvalidJsonBody)?;

            Ok(BodyJson(value))
        } else {
            Err(JsonRejection::MissingJsonContentType(""))
        }
    }
}

impl<T> core::ops::Deref for BodyJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn has_content_type<B>(
    req: &RequestParts<B>,
    expected_content_type: &str,
) -> Result<bool, JsonRejection> {
    let content_type = if let Some(content_type) = req
        .headers()
        .ok_or(JsonRejection::HeadersAlreadyExtracted)?
        .get(http::header::CONTENT_TYPE)
    {
        content_type
    } else {
        return Ok(false);
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return Ok(false);
    };

    Ok(content_type.starts_with(expected_content_type))
}
