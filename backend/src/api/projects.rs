use axum::{
    extract::{Path, Query, State},
    Form, Json,
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    dto::projects::{NewProject, Project, ProjectId, ProjectIndex, ProjectPreview},
    error::{AppError, AppResult, WithAppRejection},
    repository::projects::ProjectRepository,
    state::AppState,
    utils::{
        claims::Admin,
        pagination::{Page, Pager},
    },
};

pub fn public_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_projects))
        .routes(routes!(get_project))
}

pub fn protected_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(post_project, put_project, delete_project))
}

#[utoipa::path(
    get,
    path = "/projects",
    responses(
        (status = 200, description = "Comments found successfully", body = Page<ProjectPreview>),
    ),
)]
async fn list_projects(
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Query(pager)), _): WithAppRejection<Valid<Query<Pager<ProjectIndex>>>>,
) -> AppResult<Json<Page<ProjectPreview>>> {
    let page = repo.list(&pager).await?;
    Ok(Json(page))
}

#[utoipa::path(
    get,
    path = "/projects/{id}",
    params(
        ("id" = String, Path, description = "ID of project to fetch"),
    ),
    responses(
        (status = 200, description = "Project found successfully", body = Project),
    ),
)]
async fn get_project(
    Path(id): Path<String>,
    State(project_repo): State<ProjectRepository>,
) -> AppResult<Json<Project>> {
    let project = project_repo.read(&id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(project))
}

#[utoipa::path(
    post,
    path = "/projects",
    params(NewProject),
    responses(
        (status = 200, description = "Project posted successfully", body = ProjectId),
    ),
)]
async fn post_project(
    _: Admin,
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Form(new_project)), _): WithAppRejection<Valid<Form<NewProject>>>,
) -> AppResult<Json<ProjectId>> {
    let project = repo.create(new_project).await?;
    Ok(Json(project))
}

#[utoipa::path(
    put,
    path = "/projects/{id}",
    params(
        ("id" = String, Path, description = "ID of project to update"),
        NewProject,
    ),
    responses(
        (status = 200, description = "Project updated successfully"),
    ),
)]
async fn put_project(
    _: Admin,
    Path(id): Path<String>,
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Form(new_project)), _): WithAppRejection<Valid<Form<NewProject>>>,
) -> AppResult<()> {
    repo.update(&id, new_project).await?;
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/projects/{id}",
    params(
        ("id" = String, Path, description = "ID of project to delete"),
    ),
    responses(
        (status = 200, description = "Project deleted successfully"),
    ),
)]
async fn delete_project(
    _: Admin,
    Path(id): Path<String>,
    State(repo): State<ProjectRepository>,
) -> AppResult<()> {
    repo.delete(&id).await?;
    Ok(())
}
