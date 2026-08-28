use super::model::CommentModel;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SocialRepository;

impl SocialRepository {
    // LIKES
    pub async fn add_like(pool: &PgPool, user_id: Uuid, trick_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO trick_likes (user_id, trick_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(trick_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    pub async fn remove_like(pool: &PgPool, user_id: Uuid, trick_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM trick_likes WHERE user_id = $1 AND trick_id = $2")
            .bind(user_id)
            .bind(trick_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    pub async fn count_likes(pool: &PgPool, trick_id: Uuid) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trick_likes WHERE trick_id = $1")
            .bind(trick_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.0)
    }

    pub async fn is_liked_by_user(
        pool: &PgPool,
        user_id: Uuid,
        trick_id: Uuid,
    ) -> Result<bool, AppError> {
        let row: Option<(Uuid,)> =
            sqlx::query_as("SELECT user_id FROM trick_likes WHERE user_id = $1 AND trick_id = $2")
                .bind(user_id)
                .bind(trick_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.is_some())
    }

    // COMMENTS
    pub async fn add_comment(
        pool: &PgPool,
        user_id: Uuid,
        trick_id: Uuid,
        content: String,
    ) -> Result<CommentModel, AppError> {
        sqlx::query_as::<_, CommentModel>(
            r#"
            INSERT INTO trick_comments (user_id, trick_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, trick_id, content, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(trick_id)
        .bind(content)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn delete_comment(pool: &PgPool, comment_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM trick_comments WHERE id = $1")
            .bind(comment_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn find_comment_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<CommentModel>, AppError> {
        sqlx::query_as::<_, CommentModel>("SELECT * FROM trick_comments WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_comments_by_trick(
        pool: &PgPool,
        trick_id: Uuid,
    ) -> Result<Vec<CommentModel>, AppError> {
        sqlx::query_as::<_, CommentModel>(
            "SELECT * FROM trick_comments WHERE trick_id = $1 ORDER BY created_at DESC",
        )
        .bind(trick_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    // SPOT LIKES
    pub async fn add_spot_like(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO spot_likes (user_id, spot_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(spot_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    pub async fn remove_spot_like(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM spot_likes WHERE user_id = $1 AND spot_id = $2")
            .bind(user_id)
            .bind(spot_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    pub async fn is_spot_liked_by_user(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
    ) -> Result<bool, AppError> {
        let row: Option<(Uuid,)> =
            sqlx::query_as("SELECT user_id FROM spot_likes WHERE user_id = $1 AND spot_id = $2")
                .bind(user_id)
                .bind(spot_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.is_some())
    }

    pub async fn count_spot_likes(pool: &PgPool, spot_id: Uuid) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM spot_likes WHERE spot_id = $1")
            .bind(spot_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.0)
    }

    // SPOT COMMENTS
    pub async fn add_spot_comment(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
        content: String,
    ) -> Result<CommentModel, AppError> {
        sqlx::query_as::<_, CommentModel>(
            r#"
            INSERT INTO spot_comments (user_id, spot_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, spot_id AS trick_id, content, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(spot_id)
        .bind(content)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn find_comments_by_spot(
        pool: &PgPool,
        spot_id: Uuid,
    ) -> Result<Vec<CommentModel>, AppError> {
        sqlx::query_as::<_, CommentModel>(
            "SELECT id, user_id, spot_id AS trick_id, content, created_at, updated_at FROM spot_comments WHERE spot_id = $1 ORDER BY created_at DESC",
        )
        .bind(spot_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn delete_spot_comment(
        pool: &PgPool,
        user_id: Uuid,
        comment_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM spot_comments WHERE id = $1 AND user_id = $2")
            .bind(comment_id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
