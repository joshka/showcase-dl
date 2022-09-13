use std::str::FromStr;

use color_eyre::{eyre::Result, Report};
use futures::Future;
use reqwest::{header::HeaderValue, Client, Url};
use tracing::debug;

// Fetch a URL, applying a referer header
pub async fn fetch_with_referer(url: &str, referer: &str) -> Result<String> {
    let referer_header_value = HeaderValue::from_str(referer)?;
    let url = Url::from_str(url)?;

    tokio::spawn(async move {
        let mut referer_header_map = reqwest::header::HeaderMap::new();
        referer_header_map.insert(reqwest::header::REFERER, referer_header_value);

        let response_text = Client::new()
            .get(url)
            .headers(referer_header_map)
            .send()
            .await?
            .text()
            .await?;

        debug!(embed_response_text = ?response_text);

        Ok::<String, Report>(response_text)
    })
    .await?
}

// Await the `impl Future` if the given `Option` is `Some(_)`
#[inline]
pub async fn maybe_await(
    maybe_future: Option<impl Future<Output = Result<(), Report>>>,
) -> Result<()> {
    maybe_future.map(|fut| async {
        fut.await?;
        Ok::<(), Report>(())
    });

    Ok(())
}
