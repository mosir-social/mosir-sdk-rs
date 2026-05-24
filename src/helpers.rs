use std::time::Duration;

use reqwest::{header::HeaderMap, Client};

pub const DEFAULT_ENDPOINT: &str = "https://beta.mosir.app/api/v1";
pub const NO_MEDIA_FILE_ERROR: &str = "No media file is available for the requested media object.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewImageKind {
    Post,
    Profile,
    PostCollection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFileProfile {
    Quality,
    Compatible,
    Thumbnail,
    AnimatedCompatible,
    AnimatedThumbnail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaFileMetadata {
    pub profile: MediaFileProfile,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MediaMetadata {
    pub files: Vec<MediaFileMetadata>,
}

#[derive(Debug, Clone, Default)]
pub struct MediaFetchOptions {
    pub profile: Option<MediaFileProfile>,
    pub client: Option<Client>,
    pub headers: Option<HeaderMap>,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone, Default)]
pub struct PreviewImageFetchOptions {
    pub endpoint: Option<String>,
    pub client: Option<Client>,
    pub headers: Option<HeaderMap>,
    pub timeout: Option<Duration>,
}

const MEDIA_PROFILE_FALLBACK_ORDER: [MediaFileProfile; 5] = [
    MediaFileProfile::Quality,
    MediaFileProfile::Compatible,
    MediaFileProfile::Thumbnail,
    MediaFileProfile::AnimatedCompatible,
    MediaFileProfile::AnimatedThumbnail,
];

pub fn get_preview_image_url(kind: PreviewImageKind, id: &str, endpoint: &str) -> String {
    let route = match kind {
        PreviewImageKind::Post => "postopengraph",
        PreviewImageKind::Profile => "profileopengraph",
        PreviewImageKind::PostCollection => "collectionopengraph",
    };

    let absolute_path = format!("/ogi/{route}/{id}");

    if let Ok(base) = reqwest::Url::parse(endpoint) {
        if let Ok(url) = base.join(&absolute_path) {
            return url.to_string();
        }
    }

    format!("{}/ogi/{route}/{id}", endpoint.trim_end_matches('/'))
}

pub fn select_media_file(
    media: &MediaMetadata,
    profile: Option<MediaFileProfile>,
) -> Option<&MediaFileMetadata> {
    if let Some(profile) = profile {
        return media.files.iter().find(|file| file.profile == profile);
    }

    for candidate_profile in MEDIA_PROFILE_FALLBACK_ORDER {
        if let Some(candidate) = media
            .files
            .iter()
            .find(|file| file.profile == candidate_profile)
        {
            return Some(candidate);
        }
    }

    media.files.first()
}

pub async fn fetch_media(
    default_client: &Client,
    media: &MediaMetadata,
    options: MediaFetchOptions,
) -> anyhow::Result<reqwest::Response> {
    let file = select_media_file(media, options.profile)
        .ok_or_else(|| anyhow::anyhow!(NO_MEDIA_FILE_ERROR))?;

    let client = options.client.as_ref().unwrap_or(default_client);

    let mut req = client.get(&file.url);

    if let Some(headers) = options.headers {
        req = req.headers(headers);
    }

    if let Some(timeout) = options.timeout {
        req = req.timeout(timeout);
    }

    Ok(req.send().await?)
}

pub async fn fetch_preview_image(
    default_client: &Client,
    kind: PreviewImageKind,
    id: &str,
    options: PreviewImageFetchOptions,
) -> anyhow::Result<reqwest::Response> {
    let endpoint = options.endpoint.as_deref().unwrap_or(DEFAULT_ENDPOINT);
    let url = get_preview_image_url(kind, id, endpoint);

    let client = options.client.as_ref().unwrap_or(default_client);

    let mut req = client.get(url);

    if let Some(headers) = options.headers {
        req = req.headers(headers);
    }

    if let Some(timeout) = options.timeout {
        req = req.timeout(timeout);
    }

    Ok(req.send().await?)
}
