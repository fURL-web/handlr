use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub posts: Vec<String>,
    pub comments: Vec<String>,
}

impl User {
    pub fn new(username: &str, display_name: &str, bio: &str) -> User {
        let id = Uuid::new_v4().to_string();

        Self {
            id: id.clone(),
            username: username.to_string(),
            display_name: display_name.to_string(),
            bio: bio.to_string(),
            posts: Vec::new(),
            comments: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        create_dir_all("furl_data/users")
            .map_err(|err| format!("Error creating 'users' directory: {}", err))?;

        let file_name = format!("furl_data/users/{}.json", &self.id);
        let mut file = File::create(&file_name)
            .map_err(|err| format!("Error creating file {}: {}", &file_name, err))?;

        let json_content = serde_json::to_string_pretty(self)
            .map_err(|err| format!("Unable to serialize user to JSON: {}", err))?;
        file.write_all(json_content.as_bytes())
            .map_err(|err| format!("Error writing to file {}: {}", &file_name, err))?;

        Ok(())
    }

    pub fn load(id: &str) -> Option<User> {
        let file_name = format!("furl_data/users/{}.json", id);
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

    pub fn create_post(&mut self, post_uuid: &str) {
        self.posts.push(post_uuid.to_string());
        self.save()
            .expect("Failed to save user after creating post");
    }

    pub fn create_comment(&mut self, comment_uuid: &str) {
        self.comments.push(comment_uuid.to_string());
        self.save()
            .expect("Failed to save user after creating comment");
    }
}
