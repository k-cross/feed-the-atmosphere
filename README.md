# Feed the Atmosphere

A simple application that fetches recent posts from specific Bluesky (AT Protocol) feeds and uses AI to summarize the firehose into the top 5 most discussed topics. I wanted a tool that would categorize the most recent highly discussed topics without scrolling through hundreds of posts in the morning.

## Features

* AT Protocol Native
* Feed Caching: run `sync-feeds` to fetch saved feeds and store them locally, allowing you to query custom feeds by their display names rather than full AT-URIs
* Time-Window Filtering: filters feeds to only grab posts from desired timeframe (e.g., the last 30 minutes, the last 12 hours)
* Google Gemini-Powered Summarization: pipes the aggregated text into Gemini LLM to extract and summarize the 5 most prevalent topics

## Prerequisites

Ensure you have the following installed:
* [Rust & Cargo](https://rustup.rs/) (1.85+)

### Environment Variables
You must set the following environment variables for authentication:
* `BLUESKY_HANDLE`
* `BLUESKY_PASSWORD`
* `GEMINI_API_KEY`

## Examples

### Output

First, synchronize your saved feeds from Bluesky so you can use friendly display names:

```plaintext
export BLUESKY_HANDLE="user.bsky.social"
export BLUESKY_PASSWORD="xxxx-xxxx-xxxx-xxxx"
export GEMINI_API_KEY="AIzaSy..."

cargo run -- sync-feeds
...
Found feed: Science -> at://did:plc:.../app.bsky.feed.generator/for-science
Successfully synced feeds to your config directory!

cargo run -- list-feeds
Available feeds:
- science (at://did:plc:...)
```

Then, you can run the primary summarize command and can specify the feed using its cached display name with the number of minutes to look back (defaults: `--feed following`, `--minutes 60`).

```plaintext
cargo run -- --feed science --minutes 30

Fetching timeline for the last 30 minutes from feed: science
Found 142 posts.
Generating topic summary...

--- Top 5 Trending Topics on Your Feed ---

1. Rust 1.80 Release: Lots of discussion around the new lazy_cell stabilization and exclusive ranges.
2. The Olympics: Users live-posting reactions to the gymnastics qualifiers.
3. Tech Layoffs: General industry commentary regarding the recent game studio closures.
4. Elden Ring DLC: Spoiler-free tips on finding map fragments and beating early bosses.
5. AT Protocol Federation: Discussions about new independent PDS (Personal Data Server) instances coming online.
```
