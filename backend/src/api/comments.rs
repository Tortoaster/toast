use axum::{
    extract::{Path, State},
    Form, Json,
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    dto::{
        comments::{CommentWithUser, NewComment},
        users::User,
    },
    error::{AppError, AppResult, WithAppRejection},
    repository::comments::CommentRepository,
    state::AppState,
};

pub fn public_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(list_comments))
}

pub fn protected_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(post_comment, put_comment, delete_comment))
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}/comments",
    params(("project_id" = String, Path, description = "ID of project to list comments for")),
    responses(
        (status = 200, description = "Comments found successfully", body = [CommentWithUser]),
    ),
)]
async fn list_comments(
    Path(project_id): Path<String>,
    State(comment_repo): State<CommentRepository>,
) -> AppResult<Json<Vec<CommentWithUser>>> {
    let comments = comment_repo.list(&project_id).await?;
    Ok(Json(comments))
}

#[utoipa::path(
    post,
    path = "/projects/{project_id}/comments",
    params(
        ("project_id" = String, Path, description = "ID of project to add comment to"),
        NewComment,
    ),
    responses(
        (status = 200, description = "Comment posted successfully", body = CommentWithUser),
    ),
)]
async fn post_comment(
    user: User,
    Path(project_id): Path<String>,
    State(repo): State<CommentRepository>,
    WithRejection(Valid(new_comment), _): WithAppRejection<Valid<Form<NewComment>>>,
) -> AppResult<Json<CommentWithUser>> {
    let comment = repo.create(user.id, &project_id, &new_comment).await?;
    Ok(Json(comment))
}

#[utoipa::path(
    put,
    path = "/comments/{comment_id}",
    params(
        ("comment_id" = i32, Path, description = "ID of comment to update"),
        NewComment,
    ),
    responses(
        (status = 200, description = "Comment updated successfully", body = CommentWithUser),
    ),
)]
async fn put_comment(
    user: User,
    Path(comment_id): Path<i32>,
    State(repo): State<CommentRepository>,
    WithRejection(Valid(new_comment), _): WithAppRejection<Valid<Form<NewComment>>>,
) -> AppResult<Json<CommentWithUser>> {
    if user.is_admin || user.id == repo.read_user_id(comment_id).await? {
        let comment = repo.update(comment_id, &new_comment).await?;
        Ok(Json(comment))
    } else {
        Err(AppError::Unauthorized)
    }
}

#[utoipa::path(
    delete,
    path = "/comments/{comment_id}",
    params(
        ("comment_id" = i32, Path, description = "ID of comment to delete"),
    ),
    responses(
        (status = 200, description = "Comment deleted successfully"),
    ),
)]
async fn delete_comment(
    user: User,
    Path(comment_id): Path<i32>,
    State(repo): State<CommentRepository>,
) -> AppResult<()> {
    if user.is_admin || user.id == repo.read_user_id(comment_id).await? {
        repo.delete(comment_id).await?;
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}
