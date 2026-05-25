# mosir-sdk-rs

Rust SDK for the Mosir public GraphQL API.

## What this SDK provides

- generated Rust GraphQL types/operations from `public.graphqls` and `public.operations.graphql`
- typed operation execution with Cynic
- optional Bearer token auth
- default endpoint: `https://beta.mosir.app/api/v1`
- SSE subscription support out of the box
- media and preview helpers
- low-level connect APIs for custom SSE/event handling

## Transport choice

This SDK uses:

- `cynic` + `reqwest` for queries and mutations
- GraphQL-over-SSE over `reqwest` for subscriptions

This keeps the package small while still supporting the preferred subscription transport.
WebSocket support is intentionally not bundled.

## Install

```bash
cargo add mosir-sdk-rs
```

## Quick start

### Anonymous/public requests

Only public data needs no token.

```rust
use cynic::QueryBuilder;
use mosir_sdk_rs::{
    generated::operations::{GetPost, GetPostVariables},
    MosirClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MosirClient::new();

    let post_id = cynic::Id::new("VLO8u7UXqclQ7byjfMEX0");
    let post = client
        .run_graphql(GetPost::build(GetPostVariables { post_id: &post_id }))
        .await?;

    if let Some(data) = post.data {
        println!("{}", data.get_post.content);
    }

    Ok(())
}
```

### Authenticated requests

Use a token for authenticated operations such as notifications.

```rust
use mosir_sdk_rs::MosirClient;

let client = MosirClient::new().with_token(std::env::var("MOSIR_API_TOKEN")?);
```

## Custom endpoint

```rust
use mosir_sdk_rs::MosirClient;

let client = MosirClient::with_endpoint("https://example.com/api/v1");
```

## Common usage examples

### Get a post

```rust
use cynic::QueryBuilder;
use mosir_sdk_rs::{
    generated::operations::{GetPost, GetPostVariables},
    MosirClient,
};

let client = MosirClient::new();
let post_id = cynic::Id::new("VLO8u7UXqclQ7byjfMEX0");
let post = client
    .run_graphql(GetPost::build(GetPostVariables { post_id: &post_id }))
    .await?;

if let Some(data) = post.data {
    println!("{}", data.get_post.author.username);
    println!("{}", data.get_post.content);
}
```

### Get replies under a post

Replies are exposed as nested GraphQL fields on `Post`, so this is a good case for direct GraphQL usage:

```rust
use reqwest::Client;
use serde_json::json;

let endpoint = "https://beta.mosir.app/api/v1";
let payload = json!({
    "query": r#"
      query GetPostReplies($postId: ID!, $limit: Int) {
        getPost(postId: $postId) {
          id
          commentsRecent(limit: $limit) {
            edges {
              id
              content
              createdAt
              author {
                id
                username
                displayName
              }
            }
            pageInfo {
              endCursor
              hasNextPage
              totalCount
            }
          }
        }
      }
    "#,
    "variables": {
      "postId": "VLO8u7UXqclQ7byjfMEX0",
      "limit": 3
    }
});

let data: serde_json::Value = Client::new()
    .post(endpoint)
    .json(&payload)
    .send()
    .await?
    .json()
    .await?;

println!("{}", data["data"]["getPost"]["commentsRecent"]["edges"]);
```

### Get notifications

```rust
use cynic::QueryBuilder;
use mosir_sdk_rs::{
    generated::operations::{GetNotifications, GetNotificationsVariables},
    MosirClient,
};

let client = MosirClient::new().with_token(std::env::var("MOSIR_API_TOKEN")?);

let notifications = client
    .run_graphql(GetNotifications::build(GetNotificationsVariables {
        cursor: None,
        filter: None,
        limit: Some(20),
    }))
    .await?;

println!("{:#?}", notifications.data);
```

### Fetch media bytes from a `Media` result

```rust
use mosir_sdk_rs::MosirClient;

let client = MosirClient::new();
let response = client.fetch_media(&media, Default::default()).await?;
let bytes = response.bytes().await?;
println!("{}", bytes.len());
```

### Fetch preview image for a post, profile, or collection

```rust
use mosir_sdk_rs::{helpers::PreviewImageKind, MosirClient};

let client = MosirClient::new();

let preview_url = client.get_preview_image_url(PreviewImageKind::Post, "VLO8u7UXqclQ7byjfMEX0");
println!("{preview_url}");

let preview_response = client
    .fetch_preview_image(PreviewImageKind::Post, "VLO8u7UXqclQ7byjfMEX0", Default::default())
    .await?;
let preview_bytes = preview_response.bytes().await?;
println!("{}", preview_bytes.len());
```

All generated operations are available via `mosir_sdk_rs::generated::operations::*` and can be executed with `run_graphql(...)`.

## SSE subscriptions

Subscriptions let your app receive updates from Mosir in near real time without polling.
This SDK uses **SSE** (Server-Sent Events) for subscriptions by default.

A good example is a Discord bot:
- subscribe to `PostCreatedByAuthor`
- when a creator publishes something new, format it
- send a message into a Discord channel

That way the bot reacts as soon as something changes, instead of repeatedly calling the API every few seconds.
SSE is especially useful for backend workers, bots, notification relays, and other long-running processes that want a simple one-way stream of events from the server.
For public subscriptions like `PostCreatedByAuthor`, a token is not required.

Note: each SSE connection lasts at most 1 hour. In practice, network conditions may cause it to end earlier.
If you build a bot, worker, or relay process, make sure you implement reconnect logic.

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
        println!("{:#?}", event.data);
    }

    Ok(())
}
```

You can also use the lower-level raw subscription API:

```rust
let mut stream = client
    .subscribe_sse(
        r#"subscription PostCreatedByAuthor($authorId: ID!, $postType: PostType) {
          postCreatedByAuthor(authorId: $authorId, postType: $postType) {
            id
            content
          }
        }"#,
        Some("PostCreatedByAuthor"),
        None,
    )
    .await?;
```

## Raw GraphQL access

Authentication is optional. Pass `with_token(...)` for authenticated operations, or omit it when accessing only public data.

### Typed operation usage

```rust
use cynic::QueryBuilder;
use mosir_sdk_rs::{
    generated::operations::{GetNotifications, GetNotificationsVariables},
    MosirClient,
};

let client = MosirClient::new().with_token(std::env::var("MOSIR_API_TOKEN")?);
let data = client
    .run_graphql(GetNotifications::build(GetNotificationsVariables {
        cursor: None,
        filter: None,
        limit: Some(20),
    }))
    .await?;
```

### Raw GraphQL string usage

For ad-hoc query strings, send requests with `reqwest` directly to the same endpoint.

## WebSocket usage

WebSocket transport is not bundled.
If you want it, use your own GraphQL WebSocket client against the same endpoint.

## Notes

- default endpoint: `https://beta.mosir.app/api/v1`
- `token` is optional for public data and required only for authenticated operations
- the same applies to subscriptions: public subscription data does not require a token
- media helpers are available through `select_media_file(...)` and `fetch_media(...)`
- preview image helpers are available through `get_preview_image_url(...)` and `fetch_preview_image(...)`
- subscriptions use SSE in this SDK
- direct GraphQL usage is supported through generated typed operations and optional direct `reqwest` calls

## Development

### Generate code

```bash
task codegen
```

### Typecheck / check

```bash
task check
```

### Test

```bash
task test
```

### Smoke

```bash
task smoke
```

### Full CI check

```bash
task ci
```

## Repo artifacts

- `public.graphqls` — copied public schema artifact
- `public.operations.graphql` — copied curated operation document
- `src/generated/operations.rs` — generated GraphQL types and operations
- `src/client.rs` — thin async client and operation execution
- `src/sse.rs` — GraphQL SSE parsing/stream helpers

## License

This project is licensed under the GNU Lesser General Public License v3.0 (LGPL-3.0).
See [`LICENSE`](./LICENSE) for details.
