# Agent Documentation

This file contains architecture and technical notes intended for AI context when working on `feed-the-atmosphere`.

## Core Technologies
- **Rust Edition 2024**: The primary language for the project.
- **`clap`**: Used for CLI argument parsing. Supports optional subcommands:
  - Subcommands: `sync-feeds` (synchronizes saved feeds to local cache), `list-feeds` (lists cached feeds).
  - Main Arguments: 
    - `--feed` (default: "following"): Decides which timeline to pull from. Resolves feed names against the local cache before falling back to raw strings.
    - `--minutes` (default: 60): Used to filter the timeline for posts created within this time window.
- **`dirs`**: Used to locate the user's config directory map (e.g. `~/.config/feed-the-atmosphere/feeds.json`) for securely caching feed URIs.
- **`bsky-sdk` / `atrium-api`**: Used to communicate with the Bluesky AT Protocol. 
  - `bluesky_client::sync_user_feeds` authenticates with Bluesky and syncs the user's `SavedFeedsPrefV2` endpoint.
  - `bluesky_client::fetch_recent_posts` authenticates with Bluesky, retrieves timeline pages, and processes the raw `Record` objects extracted from `FeedViewPost`s to obtain the raw text. Wait until posts exceed the `cutoff_time` before stopping pagination.
- **`gemini-rust`**: Used for LLM summarization.
  - `summarizer::summarize_posts` compiles the fetched texts into a numbered list and asks Gemini to identify the top 5 trending topics.

## Application Architecture
- `src/main.rs`: The main entry point. Sets up `tokio::main`, parses arguments, calls the Bluesky client, and then passes the result to the summarizer.
- `src/bluesky_client.rs`: Contains the logic to create a `BskyAgent`, login via environment variables, and iterate through the `get_timeline` XRPC endpoint. 
  - Required Environment Variables: `BLUESKY_HANDLE`, `BLUESKY_PASSWORD`.
- `src/summarizer.rs`: Contains the logic to generate prompts and interface with the AI model. Includes unit tests for the prompt generation logic.
  - Required Environment Variable: `GEMINI_API_KEY`.

## Development Workflows
- Always verify changes natively using `cargo test` and `cargo check`.
- If modifying the AI prompting behavior, update the test suite in `src/summarizer.rs`.
