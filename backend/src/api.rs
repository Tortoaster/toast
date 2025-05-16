use axum::{
    extract::{Path, Query, State},
    Json,
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;

use crate::{
    dto::{Project, ProjectIndex, ProjectPreview},
    error::{AppError, AppResult, WithAppRejection},
    repository::ProjectRepository,
    utils::{Page, Pager},
};

pub async fn list_projects(
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Query(pager)), _): WithAppRejection<Valid<Query<Pager<ProjectIndex>>>>,
) -> AppResult<Json<Page<ProjectPreview>>> {
    let page = repo.list(&pager).await?;
    Ok(Json(page))
}

pub async fn get_project(
    Path(id): Path<String>,
    State(project_repo): State<ProjectRepository>,
) -> AppResult<Json<Project>> {
    let project = project_repo.read(&id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(project))
}
