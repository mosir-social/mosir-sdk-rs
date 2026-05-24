# mosir-sdk-rs

Rust SDK for the Mosir public GraphQL API.

## Status

`0.1.x` is focused on a **low-maintenance core**:

- generated GraphQL operation/types from shared schema files
- a thin async client
- required media/preview helpers (aligned with TS/Python behavior)
- typed GraphQL-SSE stream interface (`Stream<Item = Result<...>>`)
- low-level SSE connect helper (app owns reconnect strategy)

## License

Licensed under **LGPL-3.0-or-later**.

## Install

```bash
cargo add mosir-sdk-rs
```

## API overview

`MosirClient` provides:

- `run_graphql(...)` for typed Cynic operations
- `with_token(...)` for bearer auth
- `with_endpoint(...)` for custom endpoint
- `subscribe_sse_operation(...)` for typed GraphQL-SSE event streams
- `subscribe_sse(query, operation_name, ...)` for raw-query SSE event streams
- `connect_sse*` methods for low-level access to the raw `reqwest::Response`
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
            url: "https://mosir.app/",
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

Use `subscribe_sse*` as the default API. It converts GraphQL-SSE events into an async stream, so you can consume with `stream.next().await`.

For low-level control (headers/status/manual parsing), use `connect_sse*` to get the raw `reqwest::Response`.

Reconnect/backoff remains an application concern. Server-side long-lived connection limits (commonly around 1 hour) should be handled by reconnecting intentionally.

### GraphQL-SSE stream example (typed)

```rust
use cynic::{QueryBuilder, SubscriptionBuilder};
use futures_util::StreamExt;
use mosir_sdk_rs::{
    generated::operations::{
        GetAccountProfile, GetAccountProfileVariables, PostCreatedByAuthor,
        PostCreatedByAuthorVariables, PostType,
    },
    MosirClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new();

    let profile = client
        .run_graphql(GetAccountProfile::build(GetAccountProfileVariables {
            account_id: None,
            username: Some("leemiyinghao"),
        }))
        .await?;

    let author_id = &profile
        .data
        .ok_or_else(|| anyhow::anyhow!("missing profile data"))?
        .get_account_profile
        .id;

    let mut stream = client
        .subscribe_sse_operation(
            PostCreatedByAuthor::build(PostCreatedByAuthorVariables {
                author_id,
                post_type: Some(PostType::Post),
            }),
            None,
        )
        .await?;

    while let Some(event) = stream.next().await {
        let event = event?;
        println!("data: {:#?}", event.data);
        println!("errors: {:#?}", event.errors);
    }

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

The smoke example covers public GraphQL request, preview fetch, author lookup by username, and a `postCreatedByAuthor` GraphQL-SSE stream attempt with a short timeout.

## Development

```bash
task check
task test
task smoke
task ci
```
