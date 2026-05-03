use axum::{
    extract::{Path, Query, State},
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
    dto::community::{
        AcceptResponse, CreateForumPostRequest, CreateReplyRequest, ForumPostDetail,
        ForumPostListResponse, ForumPostQuery, ForumPostSummary, ForumReplyResponse, VoteResponse,
    },
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{forum_posts, forum_replies, users};

/// Build the forum router.
pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/posts", post(create_post))
        .route("/posts/:id/reply", post(create_reply))
        .route("/posts/:id/vote", put(vote_post))
        .route("/replies/:id/accept", put(accept_reply))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
        .merge(auth_routes)
}

// ─── GET /api/v1/forum/posts ─────────────────────────────────────────────────

async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ForumPostQuery>,
) -> Result<Json<ForumPostListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let mut query = forum_posts::Entity::find()
        .find_also_related(users::Entity)
        .order_by_desc(forum_posts::Column::IsPinned)
        .order_by_desc(forum_posts::Column::CreatedAt);

    if let Some(task_id) = params.task_id {
        query = query.filter(forum_posts::Column::TaskId.eq(task_id));
    }

    if let Some(ref category) = params.category {
        let cat = match category.as_str() {
            "task_help" => forum_posts::ForumCategory::TaskHelp,
            "show_and_tell" => forum_posts::ForumCategory::ShowAndTell,
            _ => forum_posts::ForumCategory::General,
        };
        query = query.filter(forum_posts::Column::Category.eq(cat));
    }

    // Count total
    let count_query = {
        let mut q = forum_posts::Entity::find();
        if let Some(task_id) = params.task_id {
            q = q.filter(forum_posts::Column::TaskId.eq(task_id));
        }
        if let Some(ref category) = params.category {
            let cat = match category.as_str() {
                "task_help" => forum_posts::ForumCategory::TaskHelp,
                "show_and_tell" => forum_posts::ForumCategory::ShowAndTell,
                _ => forum_posts::ForumCategory::General,
            };
            q = q.filter(forum_posts::Column::Category.eq(cat));
        }
        q
    };
    let total = count_query.count(&state.db).await?;

    let posts = query
        .offset((page.saturating_sub(1)) * per_page)
        .limit(per_page)
        .all(&state.db)
        .await?;

    let posts_response: Vec<ForumPostSummary> = posts
        .into_iter()
        .map(|(post, user)| {
            let username = user.map(|u| u.username).unwrap_or_default();
            ForumPostSummary {
                id: post.id,
                user_id: post.user_id,
                username,
                title: post.title,
                category: format!("{:?}", post.category).to_lowercase(),
                votes: post.votes,
                views: post.views,
                reply_count: post.reply_count,
                is_pinned: post.is_pinned,
                task_id: post.task_id,
                created_at: post.created_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(ForumPostListResponse {
        posts: posts_response,
        page,
        per_page,
        total,
    }))
}

// ─── POST /api/v1/forum/posts ────────────────────────────────────────────────

async fn create_post(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(body): Json<CreateForumPostRequest>,
) -> Result<Json<ForumPostSummary>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    if body.title.is_empty() || body.title.len() > 300 {
        return Err(AppError::Validation(
            "Title must be between 1 and 300 characters".to_string(),
        ));
    }

    if body.body_md.is_empty() {
        return Err(AppError::Validation("Body cannot be empty".to_string()));
    }

    let category = match body.category.as_deref() {
        Some("task_help") => forum_posts::ForumCategory::TaskHelp,
        Some("show_and_tell") => forum_posts::ForumCategory::ShowAndTell,
        _ => forum_posts::ForumCategory::General,
    };

    let now = chrono::Utc::now().fixed_offset();
    let post_id = Uuid::new_v4();

    let new_post = forum_posts::ActiveModel {
        id: Set(post_id),
        user_id: Set(user_id),
        title: Set(body.title.clone()),
        body_md: Set(body.body_md),
        task_id: Set(body.task_id),
        category: Set(category.clone()),
        votes: Set(0),
        views: Set(0),
        reply_count: Set(0),
        is_pinned: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    new_post.insert(&state.db).await?;

    let username = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ForumPostSummary {
        id: post_id,
        user_id,
        username,
        title: body.title,
        category: format!("{:?}", category).to_lowercase(),
        votes: 0,
        views: 0,
        reply_count: 0,
        is_pinned: false,
        task_id: body.task_id,
        created_at: now.to_rfc3339(),
    }))
}

// ─── GET /api/v1/forum/posts/:id ─────────────────────────────────────────────

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ForumPostDetail>, AppError> {
    let post = forum_posts::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // Increment views
    let mut active: forum_posts::ActiveModel = post.clone().into();
    active.views = Set(post.views + 1);
    active.update(&state.db).await?;

    let author = users::Entity::find_by_id(post.user_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    // Fetch replies with usernames
    let replies = forum_replies::Entity::find()
        .filter(forum_replies::Column::PostId.eq(id))
        .find_also_related(users::Entity)
        .order_by_asc(forum_replies::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let replies_response: Vec<ForumReplyResponse> = replies
        .into_iter()
        .map(|(reply, user)| {
            let username = user.map(|u| u.username).unwrap_or_default();
            ForumReplyResponse {
                id: reply.id,
                user_id: reply.user_id,
                username,
                body_md: reply.body_md,
                votes: reply.votes,
                is_accepted: reply.is_accepted,
                created_at: reply.created_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(ForumPostDetail {
        id: post.id,
        user_id: post.user_id,
        username: author,
        title: post.title,
        body_md: post.body_md,
        category: format!("{:?}", post.category).to_lowercase(),
        votes: post.votes,
        views: post.views + 1,
        reply_count: post.reply_count,
        is_pinned: post.is_pinned,
        task_id: post.task_id,
        created_at: post.created_at.to_rfc3339(),
        updated_at: post.updated_at.to_rfc3339(),
        replies: replies_response,
    }))
}

// ─── POST /api/v1/forum/posts/:id/reply ──────────────────────────────────────

async fn create_reply(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateReplyRequest>,
) -> Result<Json<ForumReplyResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    if body.body_md.is_empty() {
        return Err(AppError::Validation(
            "Reply body cannot be empty".to_string(),
        ));
    }

    // Verify post exists
    let post = forum_posts::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let now = chrono::Utc::now().fixed_offset();
    let reply_id = Uuid::new_v4();

    let new_reply = forum_replies::ActiveModel {
        id: Set(reply_id),
        post_id: Set(id),
        user_id: Set(user_id),
        body_md: Set(body.body_md.clone()),
        votes: Set(0),
        is_accepted: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    new_reply.insert(&state.db).await?;

    // Increment reply_count on the post
    let mut active_post: forum_posts::ActiveModel = post.clone().into();
    active_post.reply_count = Set(post.reply_count + 1);
    active_post.update(&state.db).await?;

    let username = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ForumReplyResponse {
        id: reply_id,
        user_id,
        username,
        body_md: body.body_md,
        votes: 0,
        is_accepted: false,
        created_at: now.to_rfc3339(),
    }))
}

// ─── PUT /api/v1/forum/posts/:id/vote ────────────────────────────────────────

async fn vote_post(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<VoteResponse>, AppError> {
    let post = forum_posts::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let new_votes = post.votes + 1;
    let mut active: forum_posts::ActiveModel = post.into();
    active.votes = Set(new_votes);
    active.update(&state.db).await?;

    Ok(Json(VoteResponse { votes: new_votes }))
}

// ─── PUT /api/v1/forum/replies/:id/accept ────────────────────────────────────

async fn accept_reply(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AcceptResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    let reply = forum_replies::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Reply not found".to_string()))?;

    // Verify the current user is the post author
    let post = forum_posts::Entity::find_by_id(reply.post_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if post.user_id != user_id {
        return Err(AppError::Forbidden(
            "Only the post author can accept replies".to_string(),
        ));
    }

    let mut active: forum_replies::ActiveModel = reply.into();
    active.is_accepted = Set(true);
    active.update(&state.db).await?;

    Ok(Json(AcceptResponse { is_accepted: true }))
}
