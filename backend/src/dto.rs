use serde::{Deserialize, Serialize};
use sqlx::types::{time::OffsetDateTime, uuid::Uuid};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectIndex {
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
    pub id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPreview {
    pub id: String,
    pub name: String,
    pub preview: String,
    pub thumbnail_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub thumbnail_id: Uuid,
    pub project_url: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
}
