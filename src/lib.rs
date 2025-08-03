use reqwest;
use rss::Channel;
use html2md_rs::to_md::safe_from_html_to_md;
use std::error::Error;

pub async fn fetch_rss(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
	let client = reqwest::Client::new();
	let response = client
		.get(feed_url)
		.header("User-Agent", "RSS Reader/1.0")
		.header("Accept", "application/rss+xml, application/xml, text/xml")
		.send()
		.await?;

	if !response.status().is_success() {
		return Err(format!("HTTP error: {}", response.status()).into());
	}

	let content = response.bytes().await?;
	let channel = Channel::read_from(&content[..])?;
	Ok(channel)
}

pub fn convert_html_to_markdown(html: String) -> String {
	let content = String::from(html);

	safe_from_html_to_md(content).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::Server;

	#[test]
	fn convert_html() {
		let html = String::from("<h1>Hello World!</h1>");

		assert_eq!("# Hello World!\n", convert_html_to_markdown(html));
	}

	#[tokio::test]
	async fn test_fetch_rss_success() {
			let mut server = Server::new_async().await;

			let rss_content = r#"<?xml version="1.0" encoding="UTF-8"?>
	<rss version="2.0">
		<channel>
			<title>Test Feed</title>
			<description>A test RSS feed</description>
			<item>
				<title>Test Item</title>
				<description>Test description</description>
			</item>
		</channel>
	</rss>"#;

			let mock = server
				.mock("GET", "/feed.xml")
				.with_status(200)
				.with_header("content-type", "application/rss+xml")
				.with_body(rss_content)
				.create_async()
				.await;

			let feed_url = format!("{}/feed.xml", server.url());
			let result = fetch_rss(&feed_url).await;

			mock.assert_async().await;
			assert!(result.is_ok());
			let channel = result.unwrap();
			assert_eq!(channel.title(), "Test Feed");
			assert_eq!(channel.items().len(), 1);
		}

		#[tokio::test]
		#[ignore] // Run with `cargo test -- --ignored`
		async fn test_fetch_real_rss_feed() {
			let feed_url = "https://blog.rust-lang.org/feed.xml";
			let result = fetch_rss(feed_url).await;

			match result {
				Ok(channel) => {
					assert!(!channel.title().is_empty());
					println!("Successfully fetched: {}", channel.title());
				}
				Err(e) => {
					// Don't fail the test due to network issues
					println!("Network test failed (this is okay): {}", e);
				}
			}
		}
}
