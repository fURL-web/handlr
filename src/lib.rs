use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub comment_body: String,
    pub comment_uuid: String,
    pub comment_likes: usize,
    pub comment_dislikes: usize,
}

impl Comment {
    pub fn new(comment_body: &str) -> Self {
        let comment_uuid = Uuid::new_v4().to_string();
        Self {
            comment_body: comment_body.to_string(),
            comment_uuid,
            comment_likes: 0,
            comment_dislikes: 0,
        }
    }

    pub fn save(&self, post_uuid: &str) -> Result<(), String> {
        let mut post = Post::load(post_uuid).unwrap_or_else(|| {
            panic!(
                "Post with uuid {} not found when saving comment.",
                post_uuid
            )
        });

        let comment_json = serde_json::to_value(self).unwrap();
        post.comments.push(comment_json);

        post.save().expect("Failed to save post with new comment");
        Ok(())
    }

    pub fn load(post_uuid: &str, comment_uuid: &str) -> Option<Self> {
        let post = Post::load(post_uuid)?;
        for comment_json in post.comments {
            let comment: Comment = serde_json::from_value(comment_json).ok()?;
            if comment.comment_uuid == comment_uuid {
                return Some(comment);
            }
        }
        None
    }

    pub fn update(&mut self, new_body: &str) {
        self.comment_body = new_body.to_string();
    }

    pub fn delete(&self, post_uuid: &str) {
        let mut post = Post::load(post_uuid).unwrap_or_else(|| {
            panic!(
                "Post with uuid {} not found when deleting comment.",
                post_uuid
            )
        });

        post.comments = post
            .comments
            .into_iter()
            .filter(|comment_json| {
                let comment: Comment = serde_json::from_value(comment_json.clone()).unwrap();
                comment.comment_uuid != self.comment_uuid
            })
            .collect();

        post.save()
            .expect("Failed to save post after deleting comment");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub uuid: String,
    pub comments: Vec<serde_json::Value>,
}

impl Post {
    pub fn new(title: &str, body: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            title: title.to_string(),
            body: body.to_string(),
            uuid: uuid.clone(),
            comments: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        create_dir_all("posts")
            .map_err(|err| format!("Error creating 'posts' directory: {}", err))?;

        let file_name = format!("posts/{}.json", &self.uuid);
        let mut file = File::create(&file_name)
            .map_err(|err| format!("Error creating file {}: {}", &file_name, err))?;

        let json_content = serde_json::to_string_pretty(self)
            .map_err(|err| format!("Unable to serialize post to JSON: {}", err))?;
        file.write_all(json_content.as_bytes())
            .map_err(|err| format!("Error writing to file {}: {}", &file_name, err))?;

        Ok(())
    }

    pub fn load(uuid: &str) -> Option<Self> {
        let file_name = format!("posts/{}.json", uuid);
        let mut file = match File::open(&file_name) {
            Ok(file) => file,
            Err(_) => return None, // File not found
        };

        let mut content = String::new();
        if let Err(err) = file.read_to_string(&mut content) {
            eprintln!("Error reading file {}: {}", &file_name, err);
            return None;
        }

        serde_json::from_str(&content).ok()
    }

    pub fn update(&mut self, new_title: &str, new_body: &str) {
        self.title = new_title.to_string();
        self.body = new_body.to_string();
        self.save().expect("owo"); // Save the updated content
    }

    pub fn delete(&self) {
        let file_name = format!("posts/{}.json", &self.uuid);
        if let Err(err) = std::fs::remove_file(&file_name) {
            eprintln!("Error deleting file {}: {}", &file_name, err);
        }
    }

    pub fn exists(uuid: &str) -> bool {
        let file_name = format!("posts/{}.json", uuid);
        File::open(&file_name).is_ok()
    }

    pub fn expect(self, message: &str) -> Self {
        self.save().expect(message);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn create_post() {
        let mut post = Post::new("Sample Post", "This is the body of the post.");

        post.save().expect("OWO");

        let comment = Comment::new("This is a comment.");
        comment.save(&post.uuid).expect("Failed to save comment");

        // Load and update the comment
        let loaded_comment =
            Comment::load(&post.uuid, &comment.comment_uuid).expect("Comment not found");
        let mut updated_comment = loaded_comment;
        updated_comment.update("Updated comment body");
        updated_comment
            .save(&post.uuid)
            .expect("Failed to update comment");
    }
}
