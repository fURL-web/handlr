pub mod comments;
pub mod posts;
pub mod users;

#[cfg(test)]
mod test {
    use crate::users::User;

    use super::comments::*;
    use super::posts::*;

    #[test]
    pub fn create_post_with_comments() {
        let user = User::new("thebearodactyl", "The Motherfucking Bearodactyl", "uwu");
        let post = Post::new("Sample Post", "This is the body of the post.", &user);

        post.save().expect("OWO");

        let comment = Comment::new("This is a comment.", &user);
        comment
            .save(&post.uuid, &user)
            .expect("Failed to save comment");

        // Load and update the comment
        let loaded_comment =
            Comment::load(&post.uuid, &comment.comment_uuid, &user).expect("Comment not found");
        let mut updated_comment = loaded_comment;
        updated_comment.update("Updated comment body");
        updated_comment
            .save(&post.uuid, &user)
            .expect("Failed to update comment");
    }

    #[test]
    pub fn create_post() {
        let user = User::new("thebearodactyl", "The Motherfucking Bearodactyl", "uwu");
        let post = Post::new("Sample title", "Sample body", &user);
        post.save().expect("UWU");

        println!(
            "Title: {}\n\n  Body: {}\n\nUUID: {}",
            post.title, post.body, post.uuid
        );
    }

    #[test]
    pub fn create_user() {
        let mut user = User::new("thebearodactyl", "The Motherfucking Bearodactyl", "uwu");
        user.save().expect("UWU");

        let post = Post::new("owo", "uwu", &user);
        post.save().expect("Failed to save post.");
        user.create_post(&post.uuid);

        let comment = Comment::new("comment", &user);
        comment.save(&post.uuid, &user).expect("uwu");

        user.create_comment(&comment.comment_uuid);
    }

    #[test]
    pub fn does_user_exist() {
        let user = User::new("thebearodactyl", "The Motherfucking Bearodactyl", "uwu");
        user.save().expect("UWU");

        if User::load(user.id.as_str()).is_some() {
            println!("Found user: {}", user.id);
        } else {
            eprintln!("owo");
        }
    }
}
