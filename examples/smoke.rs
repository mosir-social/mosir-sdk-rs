use std::time::Duration;

use cynic::{QueryBuilder, SubscriptionBuilder};
use futures_util::StreamExt;
use mosir_sdk_rs::{
    generated::operations::{
        GetAccountProfile, GetAccountProfileVariables, GetLinkPreview, GetLinkPreviewVariables,
        PostCreatedByAuthor, PostCreatedByAuthorVariables, PostType,
    },
    helpers::PreviewImageKind,
    MosirClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new();

    let preview_resp = client
        .run_graphql(GetLinkPreview::build(GetLinkPreviewVariables {
            url: "https://mosir.app/",
        }))
        .await?;

    if let Some(data) = preview_resp.data {
        println!("link preview: {:#?}", data.get_link_preview);
    } else {
        eprintln!("link preview query errors: {:#?}", preview_resp.errors);
    }

    let post_id = "VLO8u7UXqclQ7byjfMEX0";
    let og_url = client.get_preview_image_url(PreviewImageKind::Post, post_id);
    println!("sample preview image URL: {og_url}");

    let preview_response = client
        .fetch_preview_image(
            PreviewImageKind::Post,
            post_id,
            mosir_sdk_rs::helpers::PreviewImageFetchOptions::default(),
        )
        .await?;
    let status = preview_response.status();
    let bytes = preview_response.bytes().await?;
    println!("preview fetch status: {status}, bytes: {}", bytes.len());

    let profile_resp = client
        .run_graphql(GetAccountProfile::build(GetAccountProfileVariables {
            account_id: None,
            username: Some("leemiyinghao"),
        }))
        .await?;

    let Some(profile_data) = profile_resp.data else {
        eprintln!("profile query errors: {:#?}", profile_resp.errors);
        return Ok(());
    };

    let mut stream = client
        .subscribe_sse_operation(
            PostCreatedByAuthor::build(PostCreatedByAuthorVariables {
                author_id: &profile_data.get_account_profile.id,
                post_type: Some(PostType::Post),
            }),
            None,
        )
        .await?;

    match tokio::time::timeout(Duration::from_secs(5), stream.next()).await {
        Ok(Some(Ok(event))) => {
            println!("sse event data: {:#?}", event.data);
            println!("sse event errors: {:#?}", event.errors);
        }
        Ok(Some(Err(err))) => {
            eprintln!("sse stream error: {err}");
        }
        Ok(None) => {
            eprintln!("sse stream ended before receiving an event");
        }
        Err(_) => {
            eprintln!("sse next event timed out after 5s");
        }
    }

    Ok(())
}
