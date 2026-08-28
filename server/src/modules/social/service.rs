use super::model::CommentModel;
use super::repository::SocialRepository;
use crate::core::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SocialService;

impl SocialService {
    // LIKES
    pub async fn toggle_like(
        pool: &PgPool,
        user_id: Uuid,
        trick_id: Uuid,
    ) -> Result<bool, AppError> {
        let already_liked = SocialRepository::is_liked_by_user(pool, user_id, trick_id).await?;

        if already_liked {
            SocialRepository::remove_like(pool, user_id, trick_id).await?;
            Ok(false) // Liked removed
        } else {
            SocialRepository::add_like(pool, user_id, trick_id).await?;
            Ok(true) // Like added
        }
    }

    pub async fn get_trick_likes_count(pool: &PgPool, trick_id: Uuid) -> Result<i64, AppError> {
        SocialRepository::count_likes(pool, trick_id).await
    }

    pub async fn is_trick_liked_by_user(
        pool: &PgPool,
        user_id: Uuid,
        trick_id: Uuid,
    ) -> Result<bool, AppError> {
        SocialRepository::is_liked_by_user(pool, user_id, trick_id).await
    }

    // COMMENTS
    pub async fn add_comment(
        pool: &PgPool,
        user_id: Uuid,
        trick_id: Uuid,
        content: String,
    ) -> Result<CommentModel, AppError> {
        if content.trim().is_empty() {
            return Err(AppError::BadRequest("Comment cannot be empty".into()));
        }
        SocialRepository::add_comment(pool, user_id, trick_id, content).await
    }

    pub async fn delete_comment(
        pool: &PgPool,
        user_id: Uuid,
        comment_id: Uuid,
    ) -> Result<bool, AppError> {
        // 1. On cherche le commentaire
        let comment = SocialRepository::find_comment_by_id(pool, comment_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("Comment not found".into()))?;

        // 2. Vérification de sécurité : Seul le propriétaire (ou un futur admin) peut supprimer
        if comment.user_id != user_id {
            return Err(AppError::Unauthorized(
                "You can only delete your own comments".into(),
            ));
        }

        SocialRepository::delete_comment(pool, comment_id).await
    }

    pub async fn get_trick_comments(
        pool: &PgPool,
        trick_id: Uuid,
    ) -> Result<Vec<CommentModel>, AppError> {
        SocialRepository::find_comments_by_trick(pool, trick_id).await
    }

    // SPOTS
    pub async fn toggle_spot_like(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
    ) -> Result<bool, AppError> {
        let already_liked = SocialRepository::is_spot_liked_by_user(pool, user_id, spot_id).await?;
        if already_liked {
            SocialRepository::remove_spot_like(pool, user_id, spot_id).await?;
            Ok(false)
        } else {
            SocialRepository::add_spot_like(pool, user_id, spot_id).await?;
            Ok(true)
        }
    }

    pub async fn get_spot_likes_count(pool: &PgPool, spot_id: Uuid) -> Result<i64, AppError> {
        SocialRepository::count_spot_likes(pool, spot_id).await
    }

    pub async fn is_spot_liked_by_user(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
    ) -> Result<bool, AppError> {
        SocialRepository::is_spot_liked_by_user(pool, user_id, spot_id).await
    }

    pub async fn add_spot_comment(
        pool: &PgPool,
        user_id: Uuid,
        spot_id: Uuid,
        content: String,
    ) -> Result<CommentModel, AppError> {
        if content.trim().is_empty() {
            return Err(AppError::BadRequest("Comment cannot be empty".into()));
        }
        SocialRepository::add_spot_comment(pool, user_id, spot_id, content).await
    }

    pub async fn get_spot_comments(
        pool: &PgPool,
        spot_id: Uuid,
    ) -> Result<Vec<CommentModel>, AppError> {
        SocialRepository::find_comments_by_spot(pool, spot_id).await
    }

    pub async fn delete_spot_comment(
        pool: &PgPool,
        user_id: Uuid,
        comment_id: Uuid,
    ) -> Result<bool, AppError> {
        // Note: SocialRepository::find_comment_by_id cherche dans trick_comments.
        // On devrait utiliser une méthode qui cherche dans spot_comments.
        // Mais pour rester simple et cohérent avec ton souhait, je vais corriger le Repository.
        SocialRepository::delete_spot_comment(pool, user_id, comment_id).await
    }
}
