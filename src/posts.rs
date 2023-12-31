use super::users::User;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
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
        create_dir_all("furl_data/posts")
            .map_err(|err| format!("Error creating 'furl_data/posts' directory: {}", err))?;

        let file_name = format!("furl_data/posts/{}.json", &self.uuid);
        let mut file = File::create(&file_name)
            .map_err(|err| format!("Error creating file {}: {}", &file_name, err))?;

        let mut json_content = serde_json::to_string_pretty(self)
            .map_err(|err| format!("Unable to serialize post to JSON: {}", err))?;

        json_content.push_str("\n");
        file.write_all(json_content.as_bytes())
            .map_err(|err| format!("Error writing to file {}: {}", &file_name, err))?;

        Ok(())
    }

    pub fn load(uuid: &str, user: &User) -> Option<Self> {
        let file_name = format!("furl_data/posts/{}.json", uuid);
        let mut file = match File::open(&file_name) {
            Ok(file) => file,
            Err(_) => return None, // File not found
        };

        let mut content = String::new();
        if let Err(err) = file.read_to_string(&mut content) {
            eprintln!("Error reading file {}: {}", &file_name, err);
            return None;
        }

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
        self.save().expect("owo"); // Save the updated content
    }

    pub fn delete(&self) {
        let file_name = format!("furl_data/posts/{}.json", &self.uuid);
        if let Err(err) = std::fs::remove_file(&file_name) {
            eprintln!("Error deleting file {}: {}", &file_name, err);
        }
    }

    pub fn exists(uuid: &str) -> bool {
        let file_name = format!("furl_data/posts/{}.json", uuid);
        File::open(&file_name).is_ok()
    }

    pub fn expect(self, message: &str) -> Self {
        self.save().expect(message);
        self
    }
}
