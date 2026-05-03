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
        ArticleDetailResponse, ArticleListResponse, ArticleQuery, ArticleSummary,
        CreateArticleRequest, UpdateArticleRequest,
    },
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{articles, users};

/// Build the articles router.
pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/", post(create_article))
        .route("/:id", put(update_article))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .route("/", get(list_articles))
        .route("/:id", get(get_article))
        .merge(auth_routes)
}

// ─── GET /api/v1/articles ────────────────────────────────────────────────────

async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleQuery>,
) -> Result<Json<ArticleListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(12);

    let total = articles::Entity::find()
        .filter(articles::Column::IsPublished.eq(true))
        .count(&state.db)
        .await?;

    let items = articles::Entity::find()
        .filter(articles::Column::IsPublished.eq(true))
        .find_also_related(users::Entity)
        .order_by_desc(articles::Column::CreatedAt)
        .offset((page.saturating_sub(1)) * per_page)
        .limit(per_page)
        .all(&state.db)
        .await?;

    let articles_response: Vec<ArticleSummary> = items
        .into_iter()
        .map(|(article, user)| {
            let author_username = user.map(|u| u.username).unwrap_or_default();
            ArticleSummary {
                id: article.id,
                author_id: article.author_id,
                author_username,
                title: article.title,
                cover_image_url: article.cover_image_url,
                tags: article.tags,
                views: article.views,
                likes: article.likes,
                created_at: article.created_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(ArticleListResponse {
        articles: articles_response,
        page,
        per_page,
        total,
    }))
}

// ─── POST /api/v1/articles ───────────────────────────────────────────────────

async fn create_article(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(body): Json<CreateArticleRequest>,
) -> Result<Json<ArticleDetailResponse>, AppError> {
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

    let now = chrono::Utc::now().fixed_offset();
    let article_id = Uuid::new_v4();
    let tags = body.tags.unwrap_or_default();
    let is_published = body.is_published.unwrap_or(false);

    let new_article = articles::ActiveModel {
        id: Set(article_id),
        author_id: Set(user_id),
        title: Set(body.title.clone()),
        body_md: Set(body.body_md.clone()),
        cover_image_url: Set(body.cover_image_url.clone()),
        tags: Set(tags.clone()),
        is_published: Set(is_published),
        views: Set(0),
        likes: Set(0),
        created_at: Set(now),
        updated_at: Set(now),
    };

    new_article.insert(&state.db).await?;

    let username = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ArticleDetailResponse {
        id: article_id,
        author_id: user_id,
        author_username: username,
        title: body.title,
        body_md: body.body_md,
        cover_image_url: body.cover_image_url,
        tags,
        is_published,
        views: 0,
        likes: 0,
        created_at: now.to_rfc3339(),
        updated_at: now.to_rfc3339(),
    }))
}

// ─── GET /api/v1/articles/:id ────────────────────────────────────────────────

async fn get_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ArticleDetailResponse>, AppError> {
    let article = articles::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Increment views
    let new_views = article.views + 1;
    let mut active: articles::ActiveModel = article.clone().into();
    active.views = Set(new_views);
    active.update(&state.db).await?;

    let author = users::Entity::find_by_id(article.author_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ArticleDetailResponse {
        id: article.id,
        author_id: article.author_id,
        author_username: author,
        title: article.title,
        body_md: article.body_md,
        cover_image_url: article.cover_image_url,
        tags: article.tags,
        is_published: article.is_published,
        views: new_views,
        likes: article.likes,
        created_at: article.created_at.to_rfc3339(),
        updated_at: article.updated_at.to_rfc3339(),
    }))
}

// ─── PUT /api/v1/articles/:id ────────────────────────────────────────────────

async fn update_article(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateArticleRequest>,
) -> Result<Json<ArticleDetailResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    let article = articles::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Only author or admin can update
    let is_admin = current_user.role() == "admin";
    if article.author_id != user_id && !is_admin {
        return Err(AppError::Forbidden(
            "Only the author or admin can update this article".to_string(),
        ));
    }

    let now = chrono::Utc::now().fixed_offset();
    let mut active: articles::ActiveModel = article.clone().into();

    if let Some(title) = &body.title {
        if title.is_empty() || title.len() > 300 {
            return Err(AppError::Validation(
                "Title must be between 1 and 300 characters".to_string(),
            ));
        }
        active.title = Set(title.clone());
    }

    if let Some(body_md) = &body.body_md {
        active.body_md = Set(body_md.clone());
    }

    if let Some(cover_image_url) = &body.cover_image_url {
        active.cover_image_url = Set(Some(cover_image_url.clone()));
    }

    if let Some(tags) = &body.tags {
        active.tags = Set(tags.clone());
    }

    if let Some(is_published) = body.is_published {
        active.is_published = Set(is_published);
    }

    active.updated_at = Set(now);
    let updated = active.update(&state.db).await?;

    let author = users::Entity::find_by_id(updated.author_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ArticleDetailResponse {
        id: updated.id,
        author_id: updated.author_id,
        author_username: author,
        title: updated.title,
        body_md: updated.body_md,
        cover_image_url: updated.cover_image_url,
        tags: updated.tags,
        is_published: updated.is_published,
        views: updated.views,
        likes: updated.likes,
        created_at: updated.created_at.to_rfc3339(),
        updated_at: updated.updated_at.to_rfc3339(),
    }))
}
