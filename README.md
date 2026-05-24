# mosir-sdk-rs

Rust SDK for the Mosir public GraphQL API.

## What this SDK provides

- Generated Rust types/operations from:
  - `public.graphqls`
  - `public.operations.graphql`
- Thin async client: `MosirClient`
- Generic typed GraphQL execution: `run_graphql(...)`
- Thin SSE connection helper: `subscribe_sse(...)`
- Media/preview helpers with behavior aligned to TS/Python:
  - `get_preview_image_url`
  - `select_media_file`
  - `fetch_media`
  - `fetch_preview_image`

## Endpoint

Default endpoint:

- `https://beta.mosir.app/api/v1`

Override with:

- `MosirClient::with_endpoint("...")`

## Install

```bash
cargo add mosir-sdk-rs
```

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

## Helpers

### Preview image URL + fetch

```rust
use mosir_sdk_rs::{helpers::PreviewImageKind, MosirClient};

# async fn demo() -> anyhow::Result<()> {
let client = MosirClient::new();
let post_id = "VLO8u7UXqclQ7byjfMEX0";

let url = client.get_preview_image_url(PreviewImageKind::Post, post_id);
println!("{url}");

let response = client
    .fetch_preview_image(
        PreviewImageKind::Post,
        post_id,
        Default::default(),
    )
    .await?;

println!("status: {}", response.status());
# Ok(()) }
```

`get_preview_image_url` uses absolute OGI route semantics (same as TS `new URL("/ogi/...", endpoint)`), so URLs are rooted at endpoint origin.

### Media file selection behavior

`select_media_file` fallback order:

1. `QUALITY`
2. `COMPATIBLE`
3. `THUMBNAIL`
4. `ANIMATED_COMPATIBLE`
5. `ANIMATED_THUMBNAIL`
6. fallback to first file

If no file exists, `fetch_media` returns this exact error message:

```text
No media file is available for the requested media object.
```

## SSE note

`subscribe_sse(...)` is intentionally thin. Reconnect/backoff strategy should be implemented in your application layer.

Also note the server-side long-connection limit (commonly around 1 hour): apps should reconnect intentionally.

### Simple SSE connect example

```rust
use mosir_sdk_rs::MosirClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new().with_token("YOUR_BEARER_TOKEN");

    // Mosir SSE uses the same endpoint path as GraphQL.
    // You can pass the absolute URL directly.
    let response = client
        .subscribe_sse(client.base_url(), None)
        .await?;

    println!("status: {}", response.status());
    println!(
        "content-type: {:?}",
        response.headers().get(reqwest::header::CONTENT_TYPE)
    );

    // Keep and process the response stream in your app loop.
    Ok(())
}
```

## Smoke test

```bash
cargo run --example smoke
```

## Code generation

Generate operations from the source-of-truth GraphQL files:

```bash
task codegen
```

This updates:

- `src/generated/operations.rs`

## Development

```bash
task check
task test
task smoke
task ci
```
