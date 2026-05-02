use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Forum DTOs ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ForumPostSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub title: String,
    pub category: String,
    pub votes: i32,
    pub views: i32,
    pub reply_count: i32,
    pub is_pinned: bool,
    pub task_id: Option<Uuid>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ForumPostListResponse {
    pub posts: Vec<ForumPostSummary>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct ForumPostDetail {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub title: String,
    pub body_md: String,
    pub category: String,
    pub votes: i32,
    pub views: i32,
    pub reply_count: i32,
    pub is_pinned: bool,
    pub task_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
    pub replies: Vec<ForumReplyResponse>,
}

#[derive(Debug, Serialize)]
pub struct ForumReplyResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub body_md: String,
    pub votes: i32,
    pub is_accepted: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumPostRequest {
    pub title: String,
    pub body_md: String,
    pub task_id: Option<Uuid>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub body_md: String,
}

#[derive(Debug, Deserialize)]
pub struct ForumPostQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub task_id: Option<Uuid>,
    pub category: Option<String>,
}

// ─── Article DTOs ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ArticleSummary {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_username: String,
    pub title: String,
    pub cover_image_url: Option<String>,
    pub tags: Vec<String>,
    pub views: i32,
    pub likes: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleSummary>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct ArticleDetailResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_username: String,
    pub title: String,
    pub body_md: String,
    pub cover_image_url: Option<String>,
    pub tags: Vec<String>,
    pub is_published: bool,
    pub views: i32,
    pub likes: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub body_md: String,
    pub cover_image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub body_md: Option<String>,
    pub cover_image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

// ─── Review DTOs ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub body: String,
    pub is_featured: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewListResponse {
    pub reviews: Vec<ReviewResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i32,
    pub body: String,
}

// ─── Vote Response ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct VoteResponse {
    pub votes: i32,
}

#[derive(Debug, Serialize)]
pub struct AcceptResponse {
    pub is_accepted: bool,
}
