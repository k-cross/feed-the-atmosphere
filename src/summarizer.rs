use crate::bluesky_client::FetchedPost;
use gemini_rust::prelude::*;
use std::env;

pub fn format_prompt(posts: &[FetchedPost]) -> String {
    let mut prompt = String::from(
        "Summarize the following Bluesky timeline posts into the top 5 most discussed topics. Format it as a numbered list with a short description for each topic.\nUse reposts to help sort the recurring themes, and use likes as a potential lightly weighted filtering mechanism.\n\n",
    );
    for (i, post) in posts.iter().enumerate() {
        prompt.push_str(&format!(
            "Post {}:\nAuthor: {}\nText: {}\nLikes: {}\nReposts: {}\n\n",
            i + 1,
            post.author,
            post.text,
            post.like_count,
            post.repost_count
        ));
    }
    prompt
}

pub async fn summarize_posts(
    posts: &[FetchedPost],
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = Gemini::new(api_key).expect("Failed to create Gemini client");

    if posts.is_empty() {
        return Ok("No posts found in this timeframe.".to_string());
    }

    let prompt = format_prompt(posts);

    let response = client
        .generate_content()
        .with_user_message(&prompt)
        .execute()
        .await?;

    // Extract the text response
    let summary = response.text();

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_format_prompt() {
        let posts = vec![
            FetchedPost {
                author: "alice.bsky.social".to_string(),
                text: "Hello world!".to_string(),
                created_at: Utc::now(),
                like_count: 5,
                repost_count: 2,
            },
            FetchedPost {
                author: "bob.bsky.social".to_string(),
                text: "Rust is cool".to_string(),
                created_at: Utc::now(),
                like_count: 50,
                repost_count: 10,
            },
        ];

        let prompt = format_prompt(&posts);
        assert!(prompt.contains("Summarize the following"));
        assert!(prompt.contains("Use reposts to help sort"));
        assert!(prompt.contains(
            "Post 1:\nAuthor: alice.bsky.social\nText: Hello world!\nLikes: 5\nReposts: 2"
        ));
        assert!(prompt.contains(
            "Post 2:\nAuthor: bob.bsky.social\nText: Rust is cool\nLikes: 50\nReposts: 10"
        ));
    }
}
