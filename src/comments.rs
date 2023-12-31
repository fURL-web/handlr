use super::users::User;
use crate::posts::Post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub user_uuid: String,
    pub comment_body: String,
    pub comment_uuid: String,
    pub comment_likes: usize,
    pub comment_dislikes: usize,
}

impl Comment {
    pub fn new(comment_body: &str, user: &User) -> Self {
        let user_uuid = &user.id;
        let comment_uuid = Uuid::new_v4().to_string();
        Self {
            user_uuid: user_uuid.clone(),
            comment_body: comment_body.to_string(),
            comment_uuid,
            comment_likes: 0,
            comment_dislikes: 0,
        }
    }

    pub fn save(&self, post_uuid: &str, user_uuid: &User) -> Result<(), String> {
        let mut post = Post::load(post_uuid, user_uuid).ok_or_else(|| {
            format!(
                "Post with uuid {} not found when saving comment.",
                post_uuid
            )
        })?;

        let comment_json = serde_json::to_value(self)
            .map_err(|err| format!("Failed to serialize comment to JSON: {}", err))?;

        post.comments.push(comment_json);
        post.save()
            .map_err(|err| format!("Failed to save post with new comment: {}", err))?;

        Ok(())
    }

    pub fn load(post_uuid: &str, comment_uuid: &str, user_uuid: &User) -> Option<Self> {
        let post = Post::load(post_uuid, user_uuid)?;

        post.comments.iter().find_map(|comment_json| {
            serde_json::from_value(comment_json.clone())
                .ok()
                .and_then(|comment: Comment| {
                    if comment.comment_uuid == comment_uuid {
                        Some(comment)
                    } else {
                        None
                    }
                })
        })
    }

    pub fn update(&mut self, new_body: &str) {
        self.comment_body = new_body.to_string();
    }

    pub fn delete(&self, post_uuid: &str, user_uuid: &User) {
        let mut post = Post::load(post_uuid, user_uuid).unwrap_or_else(|| {
            panic!(
                "Post with uuid {} not found when deleting comment.",
                post_uuid
            )
        });

        post.comments.retain(|comment_json| {
            let comment: Comment = serde_json::from_value(comment_json.clone()).unwrap();
            comment.comment_uuid != self.comment_uuid
        });

        post.save()
            .expect("Failed to save post after deleting comment");
    }
}
