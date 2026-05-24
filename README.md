# mosir-sdk-rs

Rust SDK for the Mosir public GraphQL API.

## Status

`0.1.x` is focused on a **low-maintenance core**:

- generated GraphQL operation/types from shared schema files
- a thin async client
- required media/preview helpers (aligned with TS/Python behavior)
- thin SSE connect helper (app owns reconnect strategy)

## Install

```bash
cargo add mosir-sdk-rs
```

## API overview

`MosirClient` provides:

- `run_graphql(...)` for typed Cynic operations
- `with_token(...)` for bearer auth
- `with_endpoint(...)` for custom endpoint
- `subscribe_sse(query, operation_name, ...)` for thin GraphQL-SSE connection
- helper wrappers:
  - `get_preview_image_url(...)`
  - `fetch_preview_image(...)`
  - `select_media_file(...)`
  - `fetch_media(...)`

## Endpoint

Default endpoint:

- `https://beta.mosir.app/api/v1`

## Quick start

### Public request (no token)

```rust
use cynic::QueryBuilder;
use mosir_sdk_rs::{
    generated::operations::{GetLinkPreview, GetLinkPreviewVariables},
    MosirClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new();

    let response = client
        .run_graphql(GetLinkPreview::build(GetLinkPreviewVariables {
            url: "https://www.rust-lang.org/",
        }))
        .await?;

    println!("data: {:#?}", response.data);
    println!("errors: {:#?}", response.errors);
    Ok(())
}
```

### Authenticated client

```rust
use mosir_sdk_rs::MosirClient;

let client = MosirClient::new().with_token("YOUR_BEARER_TOKEN");
```

### Custom endpoint

```rust
use mosir_sdk_rs::MosirClient;

let client = MosirClient::with_endpoint("https://example.com/api/v1");
```

## Helper behavior

### Preview image URL + fetch

```rust
use mosir_sdk_rs::{helpers::PreviewImageKind, MosirClient};

# async fn demo() -> anyhow::Result<()> {
let client = MosirClient::new();
let post_id = "VLO8u7UXqclQ7byjfMEX0";

let url = client.get_preview_image_url(PreviewImageKind::Post, post_id);
println!("{url}");

let response = client
    .fetch_preview_image(PreviewImageKind::Post, post_id, Default::default())
    .await?;

println!("status: {}", response.status());
# Ok(()) }
```

`get_preview_image_url` uses absolute OGI route semantics (same as TS `new URL("/ogi/...", endpoint)`), so output is rooted at endpoint origin.

### Media file selection order

`select_media_file` fallback order:

1. `QUALITY`
2. `COMPATIBLE`
3. `THUMBNAIL`
4. `ANIMATED_COMPATIBLE`
5. `ANIMATED_THUMBNAIL`
6. first file

When no file exists, `fetch_media` returns this exact error:

```text
No media file is available for the requested media object.
```

## SSE

`subscribe_sse(...)` is intentionally thin. Reconnect/backoff should be implemented in the application.

Server-side long-lived connection limits (commonly around 1 hour) should be handled by reconnecting intentionally.

### GraphQL-SSE connect example

```rust
use mosir_sdk_rs::MosirClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new().with_token("YOUR_BEARER_TOKEN");

    let response = client
        .subscribe_sse(
            "subscription NotificationReceived { notificationReceived { __typename } }",
            Some("NotificationReceived"),
            None,
        )
        .await?;

    println!("status: {}", response.status());
    println!(
        "content-type: {:?}",
        response.headers().get(reqwest::header::CONTENT_TYPE)
    );

    Ok(())
}
```

## Code generation

Source-of-truth files:

- `public.graphqls`
- `public.operations.graphql`

Regenerate operations:

```bash
task codegen
```

Generated output:

- `src/generated/operations.rs`

## Smoke test

```bash
cargo run --example smoke
```

The smoke example covers public GraphQL request, preview fetch, and a GraphQL-SSE connect attempt with a short timeout.

## Development

```bash
task check
task test
task smoke
task ci
```
