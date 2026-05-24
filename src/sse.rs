use std::pin::Pin;

use eventsource_stream::Eventsource;
use futures_util::{future, Stream, StreamExt};
use reqwest::header::HeaderMap;

pub type GraphqlSseStream<ResponseData> =
    Pin<Box<dyn Stream<Item = anyhow::Result<cynic::GraphQlResponse<ResponseData>>> + Send>>;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphqlSseRequest<'a> {
    pub query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<&'a str>,
}

pub async fn connect<Payload: serde::Serialize + ?Sized>(
    client: &reqwest::Client,
    url: &str,
    token: Option<&str>,
    payload: &Payload,
    headers: Option<HeaderMap>,
) -> anyhow::Result<reqwest::Response> {
    let mut request = client
        .post(url)
        .header("Accept", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .json(payload);

    if let Some(token) = token {
        request = request.bearer_auth(token);
    }

    if let Some(headers) = headers {
        request = request.headers(headers);
    }

    Ok(request.send().await?)
}

pub async fn into_graphql_stream<ResponseData>(
    response: reqwest::Response,
) -> anyhow::Result<GraphqlSseStream<ResponseData>>
where
    ResponseData: serde::de::DeserializeOwned + Send + 'static,
{
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("SSE server returned {status}: {body}");
    }

    let stream = response
        .bytes_stream()
        .eventsource()
        .scan(false, |is_complete, next| {
            if *is_complete {
                return future::ready(None);
            }

            let item = match next {
                Err(err) => Some(Some(Err(anyhow::Error::new(err)))),
                Ok(event) => match event.event.as_str() {
                    "complete" => {
                        *is_complete = true;
                        None
                    }
                    "next" | "message" => {
                        let parsed = serde_json::from_str::<cynic::GraphQlResponse<ResponseData>>(
                            &event.data,
                        )
                        .map_err(anyhow::Error::from);
                        Some(Some(parsed))
                    }
                    "error" => Some(Some(Err(anyhow::anyhow!(
                        "SSE error event: {}",
                        event.data
                    )))),
                    _ => Some(None),
                },
            };

            future::ready(item)
        })
        .filter_map(|item| async move { item });

    Ok(Box::pin(stream))
}
