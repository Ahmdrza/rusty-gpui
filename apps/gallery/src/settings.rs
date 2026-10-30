//! Gallery persistence uses readable hex colors. The reusable theme crate stays
//! independent of serialization and filesystem choices.
use rusty_gpui_theme::{Color, Density, Mode, ThemeConfig};
use serde_json::{Value, json};
use std::path::PathBuf;

fn override_dir() -> Option<std::ffi::OsString> {
    std::env::var_os("RUSTY_GPUI_CONFIG_DIR").or_else(|| std::env::var_os("FORM_CONFIG_DIR"))
}

pub fn path() -> PathBuf {
    override_dir()
        .map(|dir| PathBuf::from(dir).join("theme.json"))
        .unwrap_or_else(|| default_path(false))
}

fn default_path(legacy: bool) -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    #[cfg(target_os = "macos")]
    {
        home.join("Library/Application Support")
            .join(if legacy { "Form" } else { "Rusty GPUI" })
            .join("theme.json")
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"))
            .join(if legacy { "form" } else { "rusty-gpui" })
            .join("theme.json")
    }
}

pub fn value(config: &ThemeConfig) -> Value {
    json!({"schemaVersion":1, "primary":config.primary.to_hex(), "secondary":config.secondary.map(Color::to_hex),
        "mode":if config.mode == Mode::Dark {"dark"} else {"light"},
        "density": match config.density {Density::Compact => "compact",Density::Comfortable => "comfortable",Density::Spacious => "spacious"},
        "fontFamily":config.font_family,"fontSize":config.font_size,"spacingUnit":config.spacing_unit,"radius":config.radius,"neutralTint":config.neutral_tint})
}

pub fn from_value(v: &Value) -> Result<ThemeConfig, String> {
    if v.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
        return Err("Unsupported theme schema version".into());
    }
    let mut c = ThemeConfig::default();
    if let Some(primary) = v.get("primary").and_then(Value::as_str) {
        c.primary = Color::hex(primary).map_err(|e| e.to_string())?;
    }
    if let Some(secondary) = v.get("secondary").and_then(Value::as_str) {
        c.secondary = Some(Color::hex(secondary).map_err(|e| e.to_string())?);
    }
    c.mode = if v.get("mode").and_then(Value::as_str) == Some("dark") {
        Mode::Dark
    } else {
        Mode::Light
    };
    c.density = match v.get("density").and_then(Value::as_str) {
        Some("compact") => Density::Compact,
        Some("spacious") => Density::Spacious,
        _ => Density::Comfortable,
    };
    if let Some(font) = v
        .get("fontFamily")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty() && s.len() <= 200)
    {
        c.font_family = font.into();
    }
    c.font_size = v
        .get("fontSize")
        .and_then(Value::as_f64)
        .unwrap_or(14.)
        .clamp(11., 22.) as f32;
    c.spacing_unit = v
        .get("spacingUnit")
        .and_then(Value::as_f64)
        .unwrap_or(4.)
        .clamp(2., 8.) as f32;
    c.radius = v
        .get("radius")
        .and_then(Value::as_f64)
        .unwrap_or(5.)
        .clamp(0., 20.) as f32;
    c.neutral_tint = v
        .get("neutralTint")
        .and_then(Value::as_f64)
        .unwrap_or(0.004)
        .clamp(0., 0.04) as f32;
    Ok(c)
}
pub fn load() -> ThemeConfig {
    let result = std::fs::read(path())
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound && override_dir().is_none() {
                std::fs::read(default_path(true))
            } else {
                Err(error)
            }
        })
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .and_then(|value| from_value(&value).ok());
    result.unwrap_or_default()
}
pub fn save(config: &ThemeConfig) -> Result<(), String> {
    let dest = path();
    let parent = dest.parent().ok_or("Invalid settings path")?;
    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(&value(config)).map_err(|e| e.to_string())?;
    let temp = dest.with_extension("json.tmp");
    std::fs::write(&temp, bytes).map_err(|e| e.to_string())?;
    std::fs::rename(temp, dest).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_round_trip_customization() {
        let original = ThemeConfig {
            primary: Color::hex("#247A58").unwrap(),
            secondary: Some(Color::hex("#C36A32").unwrap()),
            mode: Mode::Dark,
            density: Density::Spacious,
            font_family: "Helvetica Neue".into(),
            font_size: 18.,
            spacing_unit: 6.,
            radius: 10.,
            ..Default::default()
        };
        let next = from_value(&value(&original)).unwrap();
        assert_eq!(next.primary, original.primary);
        assert_eq!(next.secondary, original.secondary);
        assert_eq!(next.mode, original.mode);
        assert_eq!(next.density, original.density);
        assert_eq!(next.font_family, original.font_family);
        assert_eq!(next.font_size, 18.);
    }
    #[test]
    fn rejects_invalid_colors_and_bounds_numeric_values() {
        assert!(from_value(&json!({"schemaVersion":1,"primary":"wrong"})).is_err());
        assert!(from_value(&json!({"schemaVersion":2})).is_err());
        let c =
            from_value(&json!({"schemaVersion":1,"fontSize":400,"spacingUnit":-1,"radius":100}))
                .unwrap();
        assert_eq!(c.font_size, 22.);
        assert_eq!(c.spacing_unit, 2.);
        assert_eq!(c.radius, 20.);
    }
}
