use reqwest::header::HeaderMap;

pub async fn connect(
    client: &reqwest::Client,
    url: &str,
    token: Option<&str>,
    headers: Option<HeaderMap>,
) -> anyhow::Result<reqwest::Response> {
    let mut request = client
        .get(url)
        .header("Accept", "text/event-stream")
        .header("Cache-Control", "no-cache");

    if let Some(token) = token {
        request = request.bearer_auth(token);
    }

    if let Some(headers) = headers {
        request = request.headers(headers);
    }

    Ok(request.send().await?)
}
