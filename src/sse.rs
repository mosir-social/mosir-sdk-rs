use reqwest::header::HeaderMap;

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
