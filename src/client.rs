//! Thin client wrapper around cynic operations + helper utilities.

use cynic::http::ReqwestExt;
use reqwest::{header::HeaderMap, Client as HttpClient};

use crate::helpers::{
    fetch_media, fetch_preview_image, get_preview_image_url, select_media_file, MediaFetchOptions,
    MediaMetadata, PreviewImageFetchOptions, PreviewImageKind, DEFAULT_ENDPOINT,
};
use crate::sse;

#[derive(Debug, Clone)]
pub struct MosirClient {
    pub(crate) http: HttpClient,
    pub(crate) base_url: String,
    pub(crate) token: Option<String>,
}

impl MosirClient {
    pub fn new() -> Self {
        Self::with_endpoint(DEFAULT_ENDPOINT)
    }

    pub fn with_endpoint(endpoint: impl AsRef<str>) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: endpoint.as_ref().trim_end_matches('/').to_string(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn run_graphql<ResponseData, Vars>(
        &self,
        operation: cynic::Operation<ResponseData, Vars>,
    ) -> anyhow::Result<cynic::GraphQlResponse<ResponseData>>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        let mut request = self.http.post(&self.base_url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        Ok(request.run_graphql(operation).await?)
    }

    pub fn select_media_file<'a>(
        &self,
        media: &'a MediaMetadata,
        profile: Option<crate::helpers::MediaFileProfile>,
    ) -> Option<&'a crate::helpers::MediaFileMetadata> {
        select_media_file(media, profile)
    }

    pub fn get_preview_image_url(&self, kind: PreviewImageKind, id: &str) -> String {
        get_preview_image_url(kind, id, &self.base_url)
    }

    pub async fn fetch_media(
        &self,
        media: &MediaMetadata,
        options: MediaFetchOptions,
    ) -> anyhow::Result<reqwest::Response> {
        fetch_media(&self.http, media, options).await
    }

    pub async fn fetch_preview_image(
        &self,
        kind: PreviewImageKind,
        id: &str,
        options: PreviewImageFetchOptions,
    ) -> anyhow::Result<reqwest::Response> {
        let mut options = options;
        if options.endpoint.is_none() {
            options.endpoint = Some(self.base_url.clone());
        }

        fetch_preview_image(&self.http, kind, id, options).await
    }

    pub async fn subscribe_sse(
        &self,
        query: &str,
        operation_name: Option<&str>,
        headers: Option<HeaderMap>,
    ) -> anyhow::Result<reqwest::Response> {
        let payload = sse::GraphqlSseRequest {
            query,
            operation_name,
        };

        sse::connect(
            &self.http,
            &self.base_url,
            self.token.as_deref(),
            &payload,
            headers,
        )
        .await
    }

    pub async fn subscribe_sse_operation<ResponseData, Vars>(
        &self,
        operation: cynic::StreamingOperation<ResponseData, Vars>,
        headers: Option<HeaderMap>,
    ) -> anyhow::Result<reqwest::Response>
    where
        Vars: serde::Serialize,
    {
        sse::connect(
            &self.http,
            &self.base_url,
            self.token.as_deref(),
            &operation,
            headers,
        )
        .await
    }
}
