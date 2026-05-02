use axum::{
    extract::State,
    middleware,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

use crate::{
    dto::community::{CreateReviewRequest, ReviewListResponse, ReviewResponse},
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{reviews, users};

/// Build the reviews router.
pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/", post(create_review))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .route("/", get(list_reviews))
        .merge(auth_routes)
}

// ─── POST /api/v1/reviews ────────────────────────────────────────────────────

async fn create_review(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(body): Json<CreateReviewRequest>,
) -> Result<Json<ReviewResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    if body.rating < 1 || body.rating > 5 {
        return Err(AppError::Validation(
            "Rating must be between 1 and 5".to_string(),
        ));
    }

    if body.body.is_empty() {
        return Err(AppError::Validation(
            "Review body cannot be empty".to_string(),
        ));
    }

    let now = chrono::Utc::now().fixed_offset();
    let review_id = Uuid::new_v4();

    let new_review = reviews::ActiveModel {
        id: Set(review_id),
        user_id: Set(user_id),
        rating: Set(body.rating),
        body: Set(body.body.clone()),
        is_featured: Set(false),
        created_at: Set(now),
    };

    new_review.insert(&state.db).await?;

    let username = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .map(|u| u.username)
        .unwrap_or_default();

    Ok(Json(ReviewResponse {
        id: review_id,
        user_id,
        username,
        rating: body.rating,
        body: body.body,
        is_featured: false,
        created_at: now.to_rfc3339(),
    }))
}

// ─── GET /api/v1/reviews ─────────────────────────────────────────────────────

async fn list_reviews(State(state): State<AppState>) -> Result<Json<ReviewListResponse>, AppError> {
    let items = reviews::Entity::find()
        .filter(reviews::Column::IsFeatured.eq(true))
        .find_also_related(users::Entity)
        .order_by_desc(reviews::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let reviews_response: Vec<ReviewResponse> = items
        .into_iter()
        .map(|(review, user)| {
            let username = user.map(|u| u.username).unwrap_or_default();
            ReviewResponse {
                id: review.id,
                user_id: review.user_id,
                username,
                rating: review.rating,
                body: review.body,
                is_featured: review.is_featured,
                created_at: review.created_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(ReviewListResponse {
        reviews: reviews_response,
    }))
}
