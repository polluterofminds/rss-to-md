#![crate_name = "rss_to_md"]

/// This CLI tool is an easy way to fetch RSS feeds, convert them to markdown, then save them locally for offline use.

/// ## How to use
/// `cargo run -feed_url -file_path`
///
/// Example: `cargo run https://blog.rust-lang.org/feed.xml /Documents/MyRSSFeeds/`
/// Note: make sure you have created a directory for your feed items first as this CLI tool does not create a directory for you.

use trpl;
use std::fs::File;
use std::io::prelude::*;
use rss_to_md::{fetch_rss, convert_html_to_markdown};

#[derive(Debug)]
struct MarkDownContent {
    title: String,
    content: String
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let rss_url = &args[1];
        let channel = fetch_rss(&rss_url).await.unwrap();
        let mut items: Vec<MarkDownContent> = Vec::new();
        let mut missing: Vec<String> = Vec::new();

        for item in channel.items {
            let title = match item.title {
                Some(t) => t,
                None => String::from(""),
            };

            let content =
            match item.content {
                Some(html) => convert_html_to_markdown(html),
                None => {
                    println!("Could not find content for {title}");
                    String::from("")
                },
            };

            if content != "" {
                items.push(MarkDownContent{
                    title,
                    content
                });
            } else {
                missing.push(title);
            }
        }

        for i in items {
            let path = &args[2];
            let full_path = format!("{}{}{}", path, i.title, ".md");
            let mut file = File::create(full_path).expect("Failed to create file");
            file.write_all(i.content.as_bytes()).expect("Failed to write file");
        }

        for i in missing {
            let path = format!("{}{}{}", "./src/errors/", i, ".txt");
            let mut file = File::create(path).expect("Failed to create file");
            file.write_all(i.as_bytes()).expect("Failed to write file");
        }
    });
}
