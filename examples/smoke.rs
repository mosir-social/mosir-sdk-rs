use std::time::Duration;

use cynic::{QueryBuilder, SubscriptionBuilder};
use mosir_sdk_rs::{
    generated::operations::{GetLinkPreview, GetLinkPreviewVariables, NotificationReceived},
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

    let sse = tokio::time::timeout(
        Duration::from_secs(5),
        client.subscribe_sse_operation(NotificationReceived::build(()), None),
    )
    .await;

    match sse {
        Ok(Ok(response)) => {
            println!("sse connect status: {}", response.status());
            println!(
                "sse content-type: {:?}",
                response.headers().get(reqwest::header::CONTENT_TYPE)
            );
        }
        Ok(Err(err)) => {
            eprintln!("sse connect error: {err}");
        }
        Err(_) => {
            eprintln!("sse connect timed out after 5s");
        }
    }

    Ok(())
}
