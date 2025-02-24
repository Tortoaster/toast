use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    Json,
};
use axum_extra::extract::WithRejection;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    dto::projects::ProjectThumbnailId,
    error::{AppError, AppResult, WithAppRejection},
    repository::files::FileRepository,
    state::AppState,
    utils::claims::Admin,
};

pub fn protected_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(post_image))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10))
}

#[utoipa::path(post, path = "/uploads")]
async fn post_image(
    _: Admin,
    State(file_repo): State<FileRepository>,
    WithRejection(mut parts, _): WithAppRejection<Multipart>,
) -> AppResult<Json<ProjectThumbnailId>> {
    let field = parts.next_field().await?.ok_or(AppError::FileMissing)?;
    let content_type = field
        .content_type()
        .and_then(|content_type| content_type.parse().ok())
        .ok_or(AppError::FileType)?;
    let bytes = field.bytes().await?;

    if !bytes.is_empty() {
        let thumbnail_id = Uuid::new_v4();
        file_repo
            .store_thumbnail(thumbnail_id, bytes, content_type)
            .await?;
        Ok(Json(ProjectThumbnailId { thumbnail_id }))
    } else {
        Err(AppError::FileMissing)
    }
}
