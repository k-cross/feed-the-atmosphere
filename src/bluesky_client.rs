use atrium_api::types::Union;
use bsky_sdk::BskyAgent;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct FetchedPost {
    pub author: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub like_count: usize,
    pub repost_count: usize,
}

pub fn extract_post(view: &atrium_api::app::bsky::feed::defs::FeedViewPost) -> Option<FetchedPost> {
    let indexed_at = chrono::DateTime::parse_from_rfc3339(view.post.indexed_at.as_str())
        .unwrap_or_else(|_| Utc::now().into())
        .with_timezone(&Utc);

    // Extract the post text from the record.
    if let Ok(record_val) = serde_json::to_value(&view.post.record) {
        if let Ok(record) =
            serde_json::from_value::<atrium_api::app::bsky::feed::post::Record>(record_val)
        {
            return Some(FetchedPost {
                author: view.post.author.handle.to_string(),
                text: record.text.clone(),
                created_at: indexed_at,
                like_count: view.post.like_count.unwrap_or(0) as usize,
                repost_count: view.post.repost_count.unwrap_or(0) as usize,
            });
        }
    }
    None
}

pub async fn fetch_recent_posts(
    feed: &str,
    minutes: u32,
) -> Result<Vec<FetchedPost>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let handle = env::var("BLUESKY_HANDLE").expect("BLUESKY_HANDLE not set");
    let password = env::var("BLUESKY_PASSWORD").expect("BLUESKY_PASSWORD not set");

    let agent = BskyAgent::builder().build().await?;
    agent.login(&handle, &password).await?;

    let now = Utc::now();
    let cutoff_time = now - chrono::Duration::minutes(minutes as i64);

    let mut posts = Vec::new();
    let mut cursor = None;

    let cache = load_feed_cache();
    let feed_uri = if feed == "following" || feed.is_empty() {
        feed.to_string()
    } else if feed.starts_with("at://") {
        feed.to_string()
    } else if let Some(uri) = cache.get(&feed.to_lowercase()) {
        uri.to_string()
    } else {
        println!(
            "Warning: Feed '{}' not found in cache. Proceeding with literal value, which might fail.",
            feed
        );
        feed.to_string()
    };

    loop {
        let (feed_items, next_cursor) = if feed_uri == "following" || feed_uri.is_empty() {
            let params = atrium_api::app::bsky::feed::get_timeline::ParametersData {
                algorithm: None,
                cursor: cursor.clone(),
                limit: Some(100u8.try_into().unwrap()),
            };
            let result = agent.api.app.bsky.feed.get_timeline(params.into()).await?;
            (result.data.feed, result.data.cursor)
        } else {
            let params = atrium_api::app::bsky::feed::get_feed::ParametersData {
                cursor: cursor.clone(),
                feed: feed_uri.clone(),
                limit: Some(100u8.try_into().unwrap()),
            };
            let result = agent.api.app.bsky.feed.get_feed(params.into()).await?;
            (result.data.feed, result.data.cursor)
        };

        // process feed
        let mut hit_cutoff = false;

        for view in feed_items {
            let indexed_at = chrono::DateTime::parse_from_rfc3339(view.post.indexed_at.as_str())
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc);

            if indexed_at < cutoff_time {
                hit_cutoff = true;
                break;
            }

            if let Some(fetched_post) = extract_post(&view) {
                posts.push(fetched_post);
            }
        }

        if hit_cutoff || next_cursor.is_none() {
            break;
        }
        cursor = next_cursor;
    }

    Ok(posts)
}

fn get_cache_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("feed-the-atmosphere");
    fs::create_dir_all(&path).ok();
    path.push("feeds.json");
    path
}

fn load_feed_cache() -> HashMap<String, String> {
    let path = get_cache_path();
    if let Ok(contents) = fs::read_to_string(&path) {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

pub fn list_user_feeds() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let cache = load_feed_cache();
    if cache.is_empty() {
        println!("No feeds found in cache. Run `fta sync-feeds` first.");
        return Ok(());
    }
    println!("Available feeds:");
    for (name, uri) in &cache {
        println!("- {} ({})", name, uri);
    }
    Ok(())
}

pub async fn sync_user_feeds() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let handle = env::var("BLUESKY_HANDLE").expect("BLUESKY_HANDLE not set");
    let password = env::var("BLUESKY_PASSWORD").expect("BLUESKY_PASSWORD not set");

    let agent = BskyAgent::builder().build().await?;
    agent.login(&handle, &password).await?;

    let prefs = agent
        .api
        .app
        .bsky
        .actor
        .get_preferences(atrium_api::app::bsky::actor::get_preferences::ParametersData {}.into())
        .await?;
    let mut feed_uris = Vec::new();

    for pref in prefs.data.preferences {
        if let Union::Refs(item) = pref {
            match item {
                atrium_api::app::bsky::actor::defs::PreferencesItem::SavedFeedsPrefV2(v2) => {
                    for f in &v2.items {
                        if f.r#type == "feed" {
                            feed_uris.push(f.value.clone());
                        }
                    }
                }
                atrium_api::app::bsky::actor::defs::PreferencesItem::SavedFeedsPref(v1) => {
                    for f in &v1.saved {
                        feed_uris.push(f.clone());
                    }
                }
                _ => {}
            }
        }
    }

    if feed_uris.is_empty() {
        println!("No saved feeds found on your account.");
        return Ok(());
    }

    let mut feeds = HashMap::new();

    for chunk in feed_uris.chunks(25) {
        let params = atrium_api::app::bsky::feed::get_feed_generators::ParametersData {
            feeds: chunk.to_vec(),
        };
        let result = agent
            .api
            .app
            .bsky
            .feed
            .get_feed_generators(params.into())
            .await?;
        for generator in result.data.feeds {
            let name = generator.display_name.clone();
            feeds.insert(name.to_lowercase(), generator.uri.clone());
            println!("Found feed: {} -> {}", name, generator.uri);
        }
    }

    let path = get_cache_path();
    let json = serde_json::to_string_pretty(&feeds)?;
    fs::write(&path, json)?;
    println!(
        "Successfully synced {} feeds to {}",
        feeds.len(),
        path.display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_post() {
        // A valid CID for testing constraints
        let valid_cid = "bafyreiclp443lavogvhj3d2ob2cxbfuscni2k5jk7bebjzg7khl3esabwq";

        let json = format!(
            r#"{{
            "post": {{
                "uri": "at://did:plc:placeholder/app.bsky.feed.post/123",
                "cid": "{}",
                "author": {{
                    "did": "did:plc:placeholder",
                    "handle": "test.bsky.social"
                }},
                "record": {{
                    "$type": "app.bsky.feed.post",
                    "text": "This is a test post",
                    "createdAt": "2023-01-01T00:00:00Z"
                }},
                "likeCount": 10,
                "repostCount": 5,
                "indexedAt": "2023-01-01T00:00:00Z"
            }}
        }}"#,
            valid_cid
        );

        let view: atrium_api::app::bsky::feed::defs::FeedViewPost =
            serde_json::from_str(&json).expect("Failed to deserialize mock post");

        let extracted = extract_post(&view).expect("Failed to extract post");
        assert_eq!(extracted.author, "test.bsky.social");
        assert_eq!(extracted.text, "This is a test post");
        assert_eq!(extracted.like_count, 10);
        assert_eq!(extracted.repost_count, 5);
    }
}
