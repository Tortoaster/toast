use sqlx::{
    query, query_as,
    types::{time::OffsetDateTime, uuid::Uuid},
    SqlitePool,
};

use crate::{
    dto::{Project, ProjectIndex, ProjectPreview},
    error::AppResult,
    utils::{Page, Pager},
};

const DEFAULT_PROJECTS_PER_PAGE: i64 = 12;

#[derive(Clone, Debug)]
pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self, pager: &Pager<ProjectIndex>) -> sqlx::Result<Page<ProjectPreview>> {
        let items = pager.items.unwrap_or(DEFAULT_PROJECTS_PER_PAGE);
        let limit = items + 1;

        let page = match (&pager.before, &pager.after) {
            (Some(index), _) => {
                let mut transaction = self.pool.begin().await?;

                let mut previews = query_as!(
                    ProjectPreview,
                    r#"
SELECT id, name, preview, thumbnail_id AS "thumbnail_id: Uuid", date_posted AS "date_posted: OffsetDateTime"
FROM projects
WHERE NOT deleted AND (date_posted, id) > (?1, ?2)
ORDER BY date_posted, id
LIMIT ?3;
                    "#,
                    index.date_posted,
                    index.id,
                    limit,
                )
                    .fetch_all(&mut *transaction)
                    .await?;

                let has_previous = previews.len() as i64 == items + 1;
                let has_next = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE NOT deleted AND (date_posted, id) < (?1, \
                         ?2) LIMIT 1;",
                        index.date_posted,
                        index.id,
                    )
                    .fetch_optional(&mut *transaction)
                    .await?
                    .is_some();

                if has_previous {
                    previews.pop();
                }

                previews.reverse();

                transaction.commit().await?;

                Page {
                    items: previews,
                    has_previous,
                    has_next,
                }
            }
            (_, Some(index)) => {
                let mut transaction = self.pool.begin().await?;

                let mut previews = query_as!(
                    ProjectPreview,
                    r#"
SELECT id, name, preview, thumbnail_id AS "thumbnail_id: Uuid", date_posted AS "date_posted: OffsetDateTime"
FROM projects
WHERE NOT deleted AND (date_posted, id) < (?1, ?2)
ORDER BY date_posted, id DESC
LIMIT ?3;
                    "#,
                    index.date_posted,
                    index.id,
                    limit,
                )
                    .fetch_all(&mut *transaction)
                    .await?;

                let has_previous = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE NOT deleted AND (date_posted, id) > (?1, \
                         ?2) LIMIT 1;",
                        index.date_posted,
                        index.id,
                    )
                    .fetch_optional(&mut *transaction)
                    .await?
                    .is_some();
                let has_next = previews.len() as i64 == items + 1;

                if has_next {
                    previews.pop();
                }

                transaction.commit().await?;

                Page {
                    items: previews,
                    has_previous,
                    has_next,
                }
            }
            (None, None) => {
                let mut previews = query_as!(
                    ProjectPreview,
                    r#"
SELECT id, name, preview, thumbnail_id AS "thumbnail_id: Uuid", date_posted AS "date_posted: OffsetDateTime"
FROM projects
WHERE NOT deleted
ORDER BY date_posted, id DESC
LIMIT ?1;
                    "#,
                    limit,
                )
                    .fetch_all(&self.pool)
                    .await?;

                let has_next = previews.len() as i64 == items + 1;

                if has_next {
                    previews.pop();
                }

                Page {
                    items: previews,
                    has_previous: false,
                    has_next,
                }
            }
        };

        Ok(page)
    }

    pub async fn read(&self, id: &str) -> AppResult<Option<Project>> {
        let project = query_as!(
            Project,
            r#"
SELECT id, name, thumbnail_id AS "thumbnail_id: Uuid", project_url, date_posted AS "date_posted: OffsetDateTime"
FROM projects
WHERE NOT deleted AND id = ?1;
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(project)
    }
}
