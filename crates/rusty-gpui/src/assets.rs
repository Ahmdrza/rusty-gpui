//! Bundled native icons. Applications only need Rusty GPUI's Assets provider.
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets/icons"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(path
            .strip_prefix("icons/")
            .and_then(Self::get)
            .map(|asset| asset.data))
    }
    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .map(|name| format!("icons/{name}"))
            .filter(|name| name.starts_with(path))
            .map(Into::into)
            .collect())
    }
}
