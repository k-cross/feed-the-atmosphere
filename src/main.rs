mod bluesky_client;
mod summarizer;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Which feed to fetch posts from
    #[arg(short, long, default_value = "following")]
    feed: String,

    /// Timeframe in minutes to fetch posts for
    #[arg(short, long, default_value_t = 60)]
    minutes: u32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Synchronize your saved feeds from your Bluesky account to local cache
    SyncFeeds,
    /// List available feeds in your local cache
    ListFeeds,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = Args::parse();

    if let Some(cmd) = args.command {
        match cmd {
            Commands::SyncFeeds => {
                bluesky_client::sync_user_feeds().await?;
                return Ok(());
            }
            Commands::ListFeeds => {
                bluesky_client::list_user_feeds()?;
                return Ok(());
            }
        }
    }

    println!(
        "Fetching timeline for the last {} minutes from feed: {}",
        args.minutes, args.feed
    );

    let posts = bluesky_client::fetch_recent_posts(&args.feed, args.minutes).await?;
    println!("Found {} posts.", posts.len());

    println!("Generating topic summary...");
    let summary = summarizer::summarize_posts(&posts).await?;

    println!("\n--- Top 5 Trending Topics on Your Feed ---\n");
    println!("{}", summary);

    Ok(())
}
