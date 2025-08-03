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
