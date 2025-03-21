use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::{
    dto::comments::{CommentUserId, CommentWithUser, NewComment},
    error::AppResult,
    repository::users::UserRepository,
};

#[derive(Clone, Debug)]
pub struct CommentRepository {
    pool: PgPool,
    user_repo: UserRepository,
}

impl CommentRepository {
    pub fn new(pool: PgPool, user_repo: UserRepository) -> Self {
        Self { pool, user_repo }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        project_id: &str,
        comment: &NewComment,
    ) -> AppResult<CommentWithUser> {
        let CommentUserId {
            id,
            user_id,
            message,
            date_posted,
        } = query_as!(
            CommentUserId,
            "INSERT INTO comments (user_id, project_id, message) VALUES ($1, $2, $3) RETURNING \
             id, user_id, message, date_posted;",
            user_id,
            project_id,
            &comment.message,
        )
        .fetch_one(&self.pool)
        .await?;

        // TODO: Transaction
        let user = self.user_repo.get(user_id).await?;

        let comment = CommentWithUser {
            id,
            user_id: user.id,
            name: user.name,
            is_admin: user.is_admin,
            message,
            date_posted,
        };

        Ok(comment)
    }

    pub async fn list(&self, project_id: &str) -> AppResult<Vec<CommentWithUser>> {
        let comments = query_as!(
            CommentWithUser,
            "SELECT comments.id, comments.user_id, users.name, users.is_admin, comments.message, \
             comments.date_posted FROM comments INNER JOIN users ON comments.user_id = users.id \
             WHERE NOT deleted AND comments.project_id = $1 ORDER BY comments.id DESC;",
            project_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    pub async fn read_user_id(&self, id: i32) -> AppResult<Uuid> {
        pub struct UserWithId {
            user_id: Uuid,
        }

        let UserWithId { user_id } = query_as!(
            UserWithId,
            "SELECT user_id FROM comments WHERE NOT deleted AND id = $1;",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_id)
    }

    pub async fn update(&self, id: i32, comment: &NewComment) -> AppResult<CommentWithUser> {
        let CommentUserId {
            id,
            user_id,
            message,
            date_posted,
        } = query_as!(
            CommentUserId,
            "UPDATE comments SET message = $1 WHERE NOT deleted AND id = $2 RETURNING id, \
             user_id, message, date_posted;",
            &comment.message,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        // TODO: Transaction
        let user = self.user_repo.get(user_id).await?;

        let comment = CommentWithUser {
            id,
            user_id: user.id,
            name: user.name,
            is_admin: user.is_admin,
            message,
            date_posted,
        };

        Ok(comment)
    }

    pub async fn delete(&self, id: i32) -> AppResult<()> {
        query!("UPDATE comments SET deleted = TRUE WHERE id = $1;", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
