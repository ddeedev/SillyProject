use anyhow::anyhow;
use gpui::{App, AssetSource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

pub struct FontAsset {
    data: &'static [u8],
    _font_name: &'static str,
}

pub const FONT_SUPPORT: [FontAsset; 1] = [FontAsset {
    data: include_bytes!("../assets/fonts/pacifico/Pacifico-Regular.ttf"),
    _font_name: "Pacifico",
}];

#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

// TODO:: handle duplicate font name
pub fn load_font_data() -> Vec<Cow<'static, [u8]>> {
    FONT_SUPPORT
        .iter()
        .map(|font| Cow::Borrowed(font.data))
        .collect()
}
pub fn load_fonts_asset(cx: &mut App) {
    if let Err(error) = cx.text_system().add_fonts(load_font_data()) {
        eprintln!("failed to register bundled UI fonts, falling back to system fonts: {error}");
    }
}
