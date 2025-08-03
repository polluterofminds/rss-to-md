This CLI tool is an easy way to fetch RSS feeds, convert them to markdown, then save them locally for offline use.

## How to use
`cargo run -feed_url -file_path`

Example: `cargo run https://blog.rust-lang.org/feed.xml /Documents/MyRSSFeeds/`
Note: make sure you have created a directory for your feed items first as this CLI tool does not create a directory for you.
