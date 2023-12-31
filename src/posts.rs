use super::users::User;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub user_uuid: String,
    pub title: String,
    pub body: String,
    pub uuid: String,
    pub comments: Vec<serde_json::Value>,
}

impl Post {
    pub fn new(title: &str, body: &str, user: &User) -> Self {
        let user_uuid = &user.id;
        let uuid = Uuid::new_v4().to_string();
        Self {
            user_uuid: user_uuid.clone(),
            title: title.to_string(),
            body: body.to_string(),
            uuid: uuid.clone(),
            comments: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let posts_dir = "furl_data/posts";
        create_dir_all(posts_dir)
            .map_err(|err| format!("Error creating '{}' directory: {}", posts_dir, err))?;

        let file_path = format!("{}/{}.json", posts_dir, &self.uuid);
        let mut file = File::create(&file_path)
            .map_err(|err| format!("Error creating file {}: {}", &file_path, err))?;

        let json_content = serde_json::to_string_pretty(self)
            .map_err(|err| format!("Unable to serialize post to JSON: {}", err))?;

        writeln!(file, "{}", json_content)
            .map_err(|err| format!("Error writing to file {}: {}", &file_path, err))?;

        Ok(())
    }

    pub fn load(uuid: &str, user: &User) -> Option<Self> {
        let file_path = format!("furl_data/posts/{}.json", uuid);
        let mut content = String::new();
        let mut file = File::open(&file_path).ok()?;
        file.read_to_string(&mut content).ok()?;
        let post: Post = serde_json::from_str(&content).ok()?;
        if post.user_uuid == user.id {
            Some(post)
        } else {
            None
        }
    }

    pub fn update(&mut self, new_title: &str, new_body: &str) {
        self.title = new_title.to_string();
        self.body = new_body.to_string();
        self.save().expect("Error saving updated content");
    }

    pub fn delete(&self) {
        let file_path = format!("furl_data/posts/{}.json", &self.uuid);
        if let Err(err) = remove_file(&file_path) {
            eprintln!("Error deleting file {}: {}", &file_path, err);
        }
    }

    pub fn exists(uuid: &str) -> bool {
        Path::new(&format!("furl_data/posts/{}.json", uuid)).exists()
    }

    pub fn expect(self, message: &str) -> Self {
        self.save().expect(message);
        self
    }
}

