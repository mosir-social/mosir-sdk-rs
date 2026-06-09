mod schema {
    cynic::use_schema!("public.graphqls");
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetAccountProfileVariables<'a> {
    pub account_id: Option<&'a cynic::Id>,
    pub username: Option<&'a str>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetTopicFeedPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
    pub topic_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetMutualFollowersVariables<'a> {
    pub account_id: &'a cynic::Id,
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetDiscussionsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostVariables<'a> {
    pub post_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostDraftsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub filter: Option<PostDraftFilterInput>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetLinkPreviewVariables<'a> {
    pub url: &'a str,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostCollectionVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetProfileTagProfilesVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
    pub tag_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostReactionsVariables<'a> {
    pub first: Option<i32>,
    pub post_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ListProfileTagsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SearchPostCollectionsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub filter: Option<PostCollectionSearchFilterInput<'a>>,
    pub limit: Option<i32>,
    pub query: &'a str,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PostUpdatedVariables<'a> {
    pub post_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostDraftVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetMediaVariables<'a> {
    pub media_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostDraftsCountVariables {
    pub filter: Option<PostDraftFilterInput>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetFollowingPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetMyPostCollectionsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetFollowedAccountsVariables<'a> {
    pub account_id: &'a cynic::Id,
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetReactedPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
    pub reaction_type: ReactionTypeInput,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetUserReactionsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostReactionDetailsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
    pub post_id: &'a cynic::Id,
    pub reaction_type: Option<ReactionTypeInput>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PostCreatedByAuthorVariables<'a> {
    pub author_id: &'a cynic::Id,
    pub post_type: Option<PostType>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetFollowedPostCollectionsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPopularPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub language: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PostCreatedInCollectionVariables<'a> {
    pub post_collection_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PostDeletedVariables<'a> {
    pub post_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetPostCollectionsByAuthorVariables<'a> {
    pub author_id: &'a cynic::Id,
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetBlockedAccountsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetFollowingAccountsVariables<'a> {
    pub account_id: &'a cynic::Id,
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetTopicsVariables {
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetHistoryPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub include_own_posts: Option<bool>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetUserPostsVariables<'a> {
    pub account_id: &'a cynic::Id,
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
    pub post_type: Option<PostType>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetFeedPostsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetProfileTagByIdVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ReplyCreatedUnderRootPostVariables<'a> {
    pub root_post_id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetNotificationsVariables<'a> {
    pub cursor: Option<&'a str>,
    pub filter: Option<NotificationFilterInput>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Subscription",
    variables = "ReplyCreatedUnderRootPostVariables"
)]
pub struct ReplyCreatedUnderRootPost {
    #[arguments(rootPostId: $root_post_id)]
    pub reply_created_under_root_post: Post,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Subscription", variables = "PostUpdatedVariables")]
pub struct PostUpdated {
    #[arguments(postId: $post_id)]
    pub post_updated: Post,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Subscription", variables = "PostDeletedVariables")]
pub struct PostDeleted {
    #[arguments(postId: $post_id)]
    pub post_deleted: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Subscription",
    variables = "PostCreatedInCollectionVariables"
)]
pub struct PostCreatedInCollection {
    #[arguments(postCollectionID: $post_collection_id)]
    pub post_created_in_collection: Post,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Subscription",
    variables = "PostCreatedByAuthorVariables"
)]
pub struct PostCreatedByAuthor {
    #[arguments(authorId: $author_id, postType: $post_type)]
    pub post_created_by_author: Post,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Subscription")]
pub struct NotificationReceived {
    pub notification_received: Notification,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SearchPostCollectionsVariables")]
pub struct SearchPostCollections {
    #[arguments(cursor: $cursor, filter: $filter, limit: $limit, query: $query)]
    pub search_post_collections: PostCollectionSearchPage,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListProfileTagsVariables")]
pub struct ListProfileTags {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub list_profile_tags: ProfileTagConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetUserReactionsVariables")]
pub struct GetUserReactions {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_user_reactions: UserReactionsStatisticsConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UserReactionsStatisticsConnection {
    pub page_info: PageInfo,
    pub reactions: Vec<UserReactionStatistics>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UserReactionStatistics {
    pub count: i32,
    #[cynic(rename = "type")]
    pub type_: ReactionType,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetUserPostsVariables")]
pub struct GetUserPosts {
    #[arguments(accountId: $account_id, cursor: $cursor, limit: $limit, postType: $post_type)]
    pub get_user_posts: PostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct GetUnreadNotificationCount {
    pub get_unread_notification_count: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetTopicsVariables")]
pub struct GetTopics {
    #[arguments(limit: $limit)]
    pub get_topics: Vec<Topic>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Topic {
    pub id: cynic::Id,
    pub emoji: String,
    pub title: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetTopicFeedPostsVariables")]
pub struct GetTopicFeedPosts {
    #[arguments(cursor: $cursor, limit: $limit, topicId: $topic_id)]
    pub get_topic_feed_posts: PostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetReactedPostsVariables")]
pub struct GetReactedPosts {
    #[arguments(cursor: $cursor, limit: $limit, reactionType: $reaction_type)]
    pub get_reacted_posts: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetProfileTagProfilesVariables")]
pub struct GetProfileTagProfiles {
    #[arguments(cursor: $cursor, limit: $limit, tagId: $tag_id)]
    pub get_profile_tag_profiles: ProfileConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetProfileTagByIdVariables")]
pub struct GetProfileTagById {
    #[arguments(id: $id)]
    pub get_profile_tag_by_id: Option<ProfileTag>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostReactionsVariables")]
pub struct GetPostReactions {
    #[arguments(first: $first, postId: $post_id)]
    pub get_post_reactions: Option<PostReactionsConnection>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostReactionDetailsVariables")]
pub struct GetPostReactionDetails {
    #[arguments(cursor: $cursor, limit: $limit, postId: $post_id, type: $reaction_type)]
    pub get_post_reaction_details: PostReactionDetailRecordConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostDraftsCountVariables")]
pub struct GetPostDraftsCount {
    #[arguments(filter: $filter)]
    pub get_post_drafts_count: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostDraftsVariables")]
pub struct GetPostDrafts {
    #[arguments(cursor: $cursor, filter: $filter, limit: $limit)]
    pub get_post_drafts: PostDraftConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostDraftVariables")]
pub struct GetPostDraft {
    #[arguments(id: $id)]
    pub get_post_draft: Option<PostDraft>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Query",
    variables = "GetPostCollectionsByAuthorVariables"
)]
pub struct GetPostCollectionsByAuthor {
    #[arguments(authorID: $author_id, cursor: $cursor, limit: $limit)]
    pub get_post_collections_by_author: PostCollectionConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostCollectionVariables")]
pub struct GetPostCollection {
    #[arguments(id: $id)]
    pub get_post_collection: Option<PostCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPostVariables")]
pub struct GetPost {
    #[arguments(postId: $post_id)]
    pub get_post: Option<Post>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetPopularPostsVariables")]
pub struct GetPopularPosts {
    #[arguments(cursor: $cursor, language: $language, limit: $limit)]
    pub get_popular_posts: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetNotificationsVariables")]
pub struct GetNotifications {
    #[arguments(cursor: $cursor, filter: $filter, limit: $limit)]
    pub get_notifications: NotificationConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetMyPostCollectionsVariables")]
pub struct GetMyPostCollections {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_my_post_collections: PostCollectionConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetMutualFollowersVariables")]
pub struct GetMutualFollowers {
    #[arguments(accountId: $account_id, cursor: $cursor, limit: $limit)]
    pub get_mutual_followers: ProfileConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetMediaVariables")]
pub struct GetMedia {
    #[arguments(mediaId: $media_id)]
    pub get_media: MediaMetadata,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetLinkPreviewVariables")]
pub struct GetLinkPreview {
    #[arguments(url: $url)]
    pub get_link_preview: Option<LinkPreview>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetHistoryPostsVariables")]
pub struct GetHistoryPosts {
    #[arguments(cursor: $cursor, includeOwnPosts: $include_own_posts, limit: $limit)]
    pub get_history_posts: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetFollowingPostsVariables")]
pub struct GetFollowingPosts {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_following_posts: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetFollowingAccountsVariables")]
pub struct GetFollowingAccounts {
    #[arguments(accountId: $account_id, cursor: $cursor, limit: $limit)]
    pub get_following_accounts: ProfileConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Query",
    variables = "GetFollowedPostCollectionsVariables"
)]
pub struct GetFollowedPostCollections {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_followed_post_collections: PostCollectionConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetFollowedAccountsVariables")]
pub struct GetFollowedAccounts {
    #[arguments(accountId: $account_id, cursor: $cursor, limit: $limit)]
    pub get_followed_accounts: ProfileConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetFeedPostsVariables")]
pub struct GetFeedPosts {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_feed_posts: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetDiscussionsVariables")]
pub struct GetDiscussions {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_discussions: FeedPostConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct GetCurrentAccount {
    pub get_current_account: Account,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetBlockedAccountsVariables")]
pub struct GetBlockedAccounts {
    #[arguments(cursor: $cursor, limit: $limit)]
    pub get_blocked_accounts: ProfileConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetAccountProfileVariables")]
pub struct GetAccountProfile {
    #[arguments(accountId: $account_id, username: $username)]
    pub get_account_profile: Profile,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProfileTagConnection {
    pub edges: Vec<ProfileTag>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProfileConnection {
    pub edges: Vec<Profile>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Profile {
    pub id: cynic::Id,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub joined_at: DateTime,
    pub follower_count: i32,
    pub following_count: i32,
    pub prefer_interaction_type: PreferInteractionType,
    pub profile_emoji: String,
    pub profile_media_id: Option<cynic::Id>,
    pub viewer_id: Option<cynic::Id>,
    pub blocking_status: Option<BlockStatusInfo>,
    pub pinned_post_collection_id: Option<cynic::Id>,
    pub pinned_post_collection: Option<PostCollection2>,
    pub profile_follow_status: Option<ProfileFollowStatus>,
    pub profile_media: Option<MediaMetadata>,
    pub tags: Vec<ProfileTag>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProfileTag {
    pub id: cynic::Id,
    pub name: String,
    pub member_count: i32,
    #[cynic(rename = "AuthorID")]
    pub author_id: cynic::Id,
    #[cynic(rename = "Author")]
    pub author: Profile2,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProfileFollowStatus {
    pub following_level: FollowLevel,
    pub is_followed_by: bool,
    pub is_following: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostReactionDetailRecordConnection {
    pub page_info: PageInfo,
    pub reaction_records: Vec<ReactionDetailRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReactionDetailRecord {
    pub account_id: cynic::Id,
    pub created_at: DateTime,
    pub profile: Option<Profile2>,
    #[cynic(rename = "type")]
    pub type_: Option<ReactionType>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostDraftConnection {
    pub edges: Vec<PostDraft>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostDraft {
    pub id: cynic::Id,
    pub created_at: DateTime,
    pub scheduled_at: Option<DateTime>,
    pub updated_at: DateTime,
    pub post_requests: Vec<MonoPostDraft>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostConnection {
    pub edges: Vec<Post>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostCollectionSearchPage {
    pub post_collections: Vec<PostCollectionSearchResult>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostCollectionSearchResult {
    pub post_collection: PostCollection,
    pub relevance_score: Option<f64>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostCollectionConnection {
    pub edges: Vec<PostCollection>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostCollection {
    pub id: cynic::Id,
    #[cynic(rename = "authorID")]
    pub author_id: cynic::Id,
    #[cynic(rename = "coverMediaID")]
    pub cover_media_id: Option<cynic::Id>,
    pub created_at: DateTime,
    pub description: String,
    pub follow_level: FollowLevel,
    pub post_count: i32,
    pub title: String,
    pub updated_at: DateTime,
    pub author: Option<Profile2>,
    pub cover_media: Option<MediaMetadata>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct NotificationConnection {
    pub edges: Vec<Notification>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Notification {
    pub id: cynic::Id,
    pub created_at: DateTime,
    pub is_read: bool,
    pub message: Option<String>,
    #[cynic(rename = "type")]
    pub type_: NotificationType,
    #[cynic(rename = "relatedPostID")]
    pub related_post_id: Option<cynic::Id>,
    #[cynic(rename = "relatedPostCollectionID")]
    pub related_post_collection_id: Option<cynic::Id>,
    #[cynic(rename = "sourceProfileID")]
    pub source_profile_id: Option<cynic::Id>,
    #[cynic(rename = "targetProfileID")]
    pub target_profile_id: Option<cynic::Id>,
    pub reaction: Option<ReactionType>,
    pub related_post: Option<Post2>,
    pub related_post_collection: Option<PostCollection2>,
    pub source_profile: Option<Profile2>,
    pub target_profile: Option<Profile2>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct MonoPostDraft {
    pub content: String,
    pub attachment_ids: Vec<AttachmentID>,
    pub attachments: Option<Vec<Attachment>>,
    pub post_collection_ids: Vec<cynic::Id>,
    pub post_collections: Vec<PostCollection2>,
    pub post_options: Option<PostOptions>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct FeedPostConnection {
    pub edges: Vec<FeedPostEdge>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
    pub total_count: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct FeedPostEdge {
    pub id: cynic::Id,
    pub post: Post,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Post {
    pub id: cynic::Id,
    #[cynic(rename = "authorID")]
    pub author_id: cynic::Id,
    pub content: String,
    pub created_at: DateTime,
    pub parent_post_id: Option<cynic::Id>,
    pub comment_count: i32,
    pub leaf_ancestor_ids: Vec<cynic::Id>,
    pub root_ancestor_ids: Vec<cynic::Id>,
    pub author: Profile2,
    pub attachment_ids: Vec<AttachmentID>,
    pub attachments: Vec<Attachment>,
    pub post_collections: Vec<PostCollection2>,
    pub post_operation_permission: PostOperationPermission,
    pub post_options: PostOptions,
    #[arguments(first: 20)]
    pub reactions: Option<PostReactionsConnection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostReactionsConnection {
    pub post_id: cynic::Id,
    pub query_limit: i32,
    pub remaining_count: i32,
    pub viewer_id: Option<cynic::Id>,
    pub reactions: Vec<ReactionSummary>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReactionSummary {
    pub count: i32,
    pub is_reacted_by_viewer: bool,
    #[cynic(rename = "type")]
    pub type_: ReactionType,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ReactionType {
    pub emoji_value: Option<Emoji>,
    pub native_value: Option<NativeReactionType>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostOptions {
    pub comment_policy: PostCommentPolicy,
    pub copy_policy: PostCopyPolicy,
    pub reaction_policy: PostReactionPolicy,
    pub post_visibility: PostVisibility,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostVisibility {
    pub policy: PostVisibilityPolicy,
    pub profile_tags: Vec<cynic::Id>,
    pub push_policy: PostPushPolicy,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PostOperationPermission {
    pub can_comment: bool,
    pub can_react: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct BlockStatusInfo {
    pub block_status: BlockMode,
    pub is_blocked: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AttachmentID {
    #[cynic(rename = "type")]
    pub type_: AttachmentType,
    pub embedded_collection_id: Option<cynic::Id>,
    pub embedded_post_id: Option<cynic::Id>,
    pub embedded_profile_id: Option<cynic::Id>,
    pub link_url: Option<String>,
    pub media_id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Attachment {
    #[cynic(rename = "type")]
    pub type_: AttachmentType,
    pub link_preview: Option<LinkPreview>,
    pub media: Option<MediaMetadata>,
    pub embedded_profile: Option<Profile2>,
    pub embedded_post: Option<Post2>,
    pub embedded_collection: Option<PostCollection2>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "PostCollection")]
pub struct PostCollection2 {
    pub id: cynic::Id,
    #[cynic(rename = "authorID")]
    pub author_id: cynic::Id,
    #[cynic(rename = "coverMediaID")]
    pub cover_media_id: Option<cynic::Id>,
    pub created_at: DateTime,
    pub description: String,
    pub follow_level: FollowLevel,
    pub post_count: i32,
    pub title: String,
    pub updated_at: DateTime,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Post")]
pub struct Post2 {
    pub id: cynic::Id,
    #[cynic(rename = "authorID")]
    pub author_id: cynic::Id,
    pub content: String,
    pub created_at: DateTime,
    pub parent_post_id: Option<cynic::Id>,
    pub comment_count: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Profile")]
pub struct Profile2 {
    pub id: cynic::Id,
    pub username: String,
    pub display_name: Option<String>,
    pub profile_emoji: String,
    pub profile_media_id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct MediaMetadata {
    pub id: cynic::Id,
    #[cynic(rename = "accountID")]
    pub account_id: cynic::Id,
    pub aspect_ratio: Option<AspectRatio>,
    pub blur_hash: Option<String>,
    pub duration_ms: Option<i32>,
    pub files: Vec<MediaFileMetadata>,
    pub status: MediaStatus,
    #[cynic(rename = "type")]
    pub type_: MediaType,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct MediaFileMetadata {
    pub id: cynic::Id,
    pub media_id: cynic::Id,
    pub profile: MediaFileProfile,
    pub content_type: String,
    pub url: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LinkPreview {
    #[cynic(rename = "Url")]
    pub url: String,
    pub canonical_url: Option<String>,
    pub resource_type: Option<LinkPreviewResourceType>,
    pub summary: Option<String>,
    pub thumbnail_url: Option<String>,
    pub title: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AspectRatio {
    pub numerator: i32,
    pub denominator: i32,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Account {
    pub id: cynic::Id,
    pub username: String,
    pub email: String,
    pub created_at: DateTime,
    pub last_login: Option<DateTime>,
    pub status: Status,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum AttachmentType {
    Collection,
    Link,
    Media,
    Post,
    Profile,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum BlockMode {
    BiodirectionalBlock,
    Block,
    None,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum FollowLevel {
    Following,
    None,
    Notify,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum LinkPreviewResourceType {
    #[cynic(rename = "InstagramPost")]
    InstagramPost,
    Unknown,
    #[cynic(rename = "YouTubeShorts")]
    YouTubeShorts,
    #[cynic(rename = "YouTubeVideo")]
    YouTubeVideo,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum MediaFileProfile {
    AnimatedCompatible,
    AnimatedThumbnail,
    Compatible,
    Quality,
    Thumbnail,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum MediaStatus {
    Failed,
    Pending,
    Ready,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum MediaType {
    Audio,
    Image,
    Video,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum NativeReactionType {
    Bookmark,
    Like,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum NotificationType {
    Follow,
    Mention,
    NewPost,
    PostCollectionFollow,
    React,
    Reply,
    SystemUpdate,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostCommentPolicy {
    AllowAll,
    AllowFollowers,
    OnlyAuthor,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostCopyPolicy {
    Allow,
    Disallow,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostPushPolicy {
    Followers,
    Public,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostReactionPolicy {
    AllowAll,
    AllowFollowers,
    OnlyAuthor,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostType {
    Post,
    Reply,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PostVisibilityPolicy {
    Followers,
    LoggedInOnly,
    MutualFollowers,
    Public,
    TagMembers,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PreferInteractionType {
    Both,
    Comments,
    DoNotDisturb,
    None,
    Reactions,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum Status {
    Active,
    Deleted,
    Inactive,
    Suspended,
}

#[derive(cynic::InputObject, Debug)]
pub struct ReactionTypeInput {
    pub emoji_value: Option<Emoji>,
    pub native_value: Option<NativeReactionType>,
}

#[derive(cynic::InputObject, Debug)]
pub struct PostDraftFilterInput {
    pub is_scheduled: Option<bool>,
}

#[derive(cynic::InputObject, Debug)]
pub struct PostCollectionSearchFilterInput<'a> {
    #[cynic(rename = "authorID")]
    pub author_id: Option<&'a cynic::Id>,
}

#[derive(cynic::InputObject, Debug)]
pub struct NotificationFilterInput {
    pub read: Option<bool>,
    pub types: Option<Vec<NotificationType>>,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Emoji(pub String);
