//! Configurable semantic tokens, independent of GPUI and of any application.
//! Colors are generated in OKLCH, gamut mapped, and checked in sRGB for contrast.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidColor;

impl fmt::Display for InvalidColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Expected a six-digit hex color, for example #4567D5")
    }
}
impl std::error::Error for InvalidColor {}

impl Color {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn hex(value: &str) -> Result<Self, InvalidColor> {
        let value = value.strip_prefix('#').unwrap_or(value);
        if value.len() != 6 || !value.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err(InvalidColor);
        }
        let n = u32::from_str_radix(value, 16).map_err(|_| InvalidColor)?;
        Ok(Self::rgb(
            ((n >> 16) & 255) as f32 / 255.0,
            ((n >> 8) & 255) as f32 / 255.0,
            (n & 255) as f32 / 255.0,
        ))
    }
    pub fn to_u32(self) -> u32 {
        let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
        (channel(self.r) << 16) | (channel(self.g) << 8) | channel(self.b)
    }
    pub fn to_hex(self) -> String {
        format!("#{:06X}", self.to_u32())
    }
    pub fn luminance(self) -> f32 {
        0.2126 * linear(self.r) + 0.7152 * linear(self.g) + 0.0722 * linear(self.b)
    }
    pub fn contrast(self, other: Self) -> f32 {
        let a = self.luminance();
        let b = other.luminance();
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
    pub fn on_color(self) -> Self {
        if self.contrast(Self::BLACK) >= self.contrast(Self::WHITE) {
            Self::BLACK
        } else {
            Self::WHITE
        }
    }
    pub fn oklch(self) -> [f32; 3] {
        let r = linear(self.r);
        let g = linear(self.g);
        let b = linear(self.b);
        let l = (0.41222146 * r + 0.53633255 * g + 0.051445995 * b).cbrt();
        let m = (0.2119035 * r + 0.6806995 * g + 0.10739696 * b).cbrt();
        let s = (0.08830246 * r + 0.28171885 * g + 0.6299787 * b).cbrt();
        let light = 0.21045426 * l + 0.7936178 * m - 0.004072047 * s;
        let a = 1.9779985 * l - 2.4285922 * m + 0.4505937 * s;
        let b = 0.025904037 * l + 0.78277177 * m - 0.80867577 * s;
        [light, a.hypot(b), b.atan2(a)]
    }
    pub fn from_oklch(l: f32, c: f32, h: f32) -> Self {
        // Binary-search chroma instead of clipping channels, preserving hue.
        let l = l.clamp(0.0, 1.0);
        let mut low = 0.0;
        let mut high = c.max(0.0);
        for _ in 0..24 {
            let mid = (low + high) * 0.5;
            let rgb = oklch_linear(l, mid, h);
            if rgb.iter().all(|v| *v >= -0.00001 && *v <= 1.00001) {
                low = mid;
            } else {
                high = mid;
            }
        }
        let rgb = oklch_linear(l, low, h);
        Self::rgb(
            encoded(rgb[0]).clamp(0.0, 1.0),
            encoded(rgb[1]).clamp(0.0, 1.0),
            encoded(rgb[2]).clamp(0.0, 1.0),
        )
    }
    pub fn scale(self) -> [Self; 11] {
        let [_, c, h] = self.oklch();
        [
            0.98, 0.95, 0.90, 0.83, 0.74, 0.64, 0.54, 0.44, 0.34, 0.25, 0.17,
        ]
        .map(|l| Self::from_oklch(l, c, h))
    }
}

fn linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}
fn encoded(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}
fn oklch_linear(l: f32, c: f32, h: f32) -> [f32; 3] {
    let a = c * h.cos();
    let b = c * h.sin();
    let ll = (l + 0.39633778 * a + 0.21580376 * b).powi(3);
    let m = (l - 0.105561346 * a - 0.06385417 * b).powi(3);
    let s = (l - 0.08948418 * a - 1.2914855 * b).powi(3);
    [
        4.0767417 * ll - 3.3077116 * m + 0.23096994 * s,
        -1.268438 * ll + 2.6097574 * m - 0.3413194 * s,
        -0.0041960863 * ll - 0.7034186 * m + 1.7076147 * s,
    ]
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Light,
    Dark,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}
impl Density {
    pub fn control_height(self) -> f32 {
        match self {
            Self::Compact => 28.0,
            Self::Comfortable => 36.0,
            Self::Spacious => 44.0,
        }
    }
    pub fn factor(self) -> f32 {
        match self {
            Self::Compact => 0.8,
            Self::Comfortable => 1.0,
            Self::Spacious => 1.2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeConfig {
    pub primary: Color,
    /// None generates a related secondary at a 60-degree hue offset.
    pub secondary: Option<Color>,
    pub mode: Mode,
    pub density: Density,
    pub font_family: String,
    pub font_size: f32,
    pub spacing_unit: f32,
    pub radius: f32,
    pub neutral_tint: f32,
}
impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            primary: Color::hex("#4567D5").unwrap(),
            secondary: None,
            mode: Mode::Light,
            density: Density::Comfortable,
            font_family: "Inter".into(),
            font_size: 14.0,
            spacing_unit: 4.0,
            radius: 5.0,
            neutral_tint: 0.004,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokens {
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub foreground: Color,
    pub muted: Color,
    pub border: Color,
    pub subtle: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub on_primary: Color,
    pub primary_subtle: Color,
    pub primary_text: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub focus: Color,
    pub scale: [Color; 11],
    pub secondary_scale: [Color; 11],
    pub control_height: f32,
    pub control_padding: f32,
    pub spacing: [f32; 9],
    pub type_scale: [f32; 7],
    pub radius: f32,
    pub font_family: String,
    pub font_size: f32,
}

impl ThemeConfig {
    pub fn generate(&self) -> Tokens {
        let dark = self.mode == Mode::Dark;
        let [_, chroma, hue] = self.primary.oklch();
        let neutral =
            |l| Color::from_oklch(l, finite_or(self.neutral_tint, 0.004).clamp(0.0, 0.04), hue);
        let background = neutral(if dark { 0.19 } else { 0.985 });
        let surface = neutral(if dark { 0.23 } else { 1.0 });
        let foreground = neutral(if dark { 0.94 } else { 0.22 });
        let subtle = neutral(if dark { 0.27 } else { 0.955 });
        let primary_subtle =
            Color::from_oklch(if dark { 0.30 } else { 0.95 }, chroma.min(0.04), hue);
        let muted = ensure_contrast(neutral(if dark { 0.67 } else { 0.50 }), subtle, 4.6);
        let primary = self.primary;
        let primary_text = ensure_contrast(primary, primary_subtle, 4.6);
        let focus = ensure_contrast(primary, background, 3.0);
        let secondary = self.secondary.unwrap_or_else(|| {
            Color::from_oklch(
                if dark { 0.72 } else { 0.49 },
                chroma * 0.55,
                hue + std::f32::consts::PI / 3.0,
            )
        });
        let font_size = finite_or(self.font_size, 14.0).clamp(11.0, 22.0);
        let unit = finite_or(self.spacing_unit, 4.0).clamp(2.0, 8.0);
        let control_height = self
            .density
            .control_height()
            .max(font_size * 1.5 + unit * 2.0);
        let semantic = |hex: &str| ensure_contrast(Color::hex(hex).unwrap(), subtle, 4.6);
        Tokens {
            background,
            surface,
            elevated: neutral(if dark { 0.27 } else { 1.0 }),
            foreground,
            muted,
            border: neutral(if dark { 0.36 } else { 0.88 }),
            subtle,
            primary,
            primary_hover: ensure_contrast(
                Color::from_oklch(
                    (primary.oklch()[0] + if dark { 0.06 } else { -0.06 }).clamp(0.15, 0.92),
                    chroma,
                    hue,
                ),
                primary.on_color(),
                4.6,
            ),
            on_primary: primary.on_color(),
            primary_subtle,
            primary_text,
            secondary,
            on_secondary: secondary.on_color(),
            success: semantic("#238451"),
            warning: semantic("#A86A13"),
            danger: semantic("#CB3A3A"),
            focus,
            scale: primary.scale(),
            secondary_scale: secondary.scale(),
            control_height,
            control_padding: unit * 3.0 * self.density.factor(),
            spacing: [0.0, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, 16.0]
                .map(|n| n * unit * self.density.factor()),
            type_scale: [0.7857, 0.8571, 1.0, 1.1429, 1.4286, 2.0, 3.1429].map(|n| n * font_size),
            radius: finite_or(self.radius, 5.0).clamp(0.0, 20.0),
            font_family: self.font_family.clone(),
            font_size,
        }
    }
}
fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

/// Adjust only OKLCH lightness until text clears the requested contrast ratio.
pub fn ensure_contrast(color: Color, background: Color, target: f32) -> Color {
    if color.contrast(background) >= target {
        return color;
    }
    let [l, c, h] = color.oklch();
    let end = if background.contrast(Color::BLACK) > background.contrast(Color::WHITE) {
        0.0
    } else {
        1.0
    };
    for step in 1..=100 {
        let candidate = Color::from_oklch(l + (end - l) * step as f32 / 100.0, c, h);
        if candidate.contrast(background) >= target {
            return candidate;
        }
    }
    background.on_color()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_strict_hex() {
        assert_eq!(Color::hex("#4567d5").unwrap().to_hex(), "#4567D5");
        for bad in ["#fff", "1234567", "#GGGGGG", "ééé"] {
            assert!(Color::hex(bad).is_err());
        }
    }
    #[test]
    fn neutral_and_extreme_seeds_have_readable_text() {
        for mode in [Mode::Light, Mode::Dark] {
            for hex in [
                "#000000", "#FFFFFF", "#FFFF00", "#0000FF", "#4567D5", "#FF00FF", "#00FF00",
                "#777777",
            ] {
                let t = ThemeConfig {
                    primary: Color::hex(hex).unwrap(),
                    mode,
                    ..Default::default()
                }
                .generate();
                assert!(t.primary.contrast(t.on_primary) >= 4.5, "{hex}");
                assert!(t.primary_hover.contrast(t.on_primary) >= 4.5, "{hex}");
                assert!(t.primary_text.contrast(t.background) >= 4.5, "{hex}");
                assert!(t.primary_text.contrast(t.primary_subtle) >= 4.5, "{hex}");
                assert!(t.muted.contrast(t.background) >= 4.5);
                assert!(t.focus.contrast(t.background) >= 3.0);
                for color in t.scale {
                    assert!(
                        [color.r, color.g, color.b]
                            .iter()
                            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
                    );
                }
            }
        }
    }
    #[test]
    fn spacing_and_type_expand_without_clipping_controls() {
        let small = ThemeConfig::default().generate();
        let large = ThemeConfig {
            font_size: 22.0,
            spacing_unit: 8.0,
            density: Density::Spacious,
            ..Default::default()
        }
        .generate();
        assert!(large.spacing[4] > small.spacing[4]);
        assert!(large.control_height >= large.font_size * 1.5 + 16.0);
    }
    #[test]
    fn explicit_secondary_is_preserved() {
        let secondary = Color::hex("#D57145").unwrap();
        assert_eq!(
            ThemeConfig {
                secondary: Some(secondary),
                ..Default::default()
            }
            .generate()
            .secondary,
            secondary
        );
    }
    #[test]
    fn scales_are_monotonic_and_color_round_trips() {
        let c = Color::hex("#4567D5").unwrap();
        let [l, ch, h] = c.oklch();
        let recovered = Color::from_oklch(l, ch, h);
        assert!((recovered.r - c.r).abs() < 0.002);
        assert!(
            c.scale()
                .windows(2)
                .all(|s| s[0].luminance() > s[1].luminance())
        );
    }
}
