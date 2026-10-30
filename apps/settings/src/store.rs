use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub workspace: String,
    pub updates: bool,
    pub dark_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            workspace: "My workspace".into(),
            updates: false,
            dark_mode: false,
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.workspace.trim().is_empty() {
            return Err("Enter a workspace name.");
        }
        if self.workspace.chars().count() > 80 {
            return Err("Use 80 characters or fewer.");
        }
        if self.workspace.chars().any(char::is_control) {
            return Err("Use a name without control characters.");
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    pub fn load(&self) -> Result<Option<Settings>, String> {
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(format!("Could not read settings: {error}")),
        };
        let settings: Settings = serde_json::from_slice(&bytes)
            .map_err(|error| format!("Could not read settings: {error}"))?;
        settings.validate().map_err(str::to_owned)?;
        Ok(Some(settings))
    }

    pub fn save(&self, settings: &Settings) -> Result<(), String> {
        settings.validate().map_err(str::to_owned)?;
        let parent = self.path.parent().ok_or("Invalid settings location.")?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("Could not create settings folder: {error}"))?;
        // A sibling temporary file makes replacement atomic on the same filesystem.
        // A failed write never truncates the previous settings file.
        let result = (|| -> io::Result<()> {
            use std::io::Write;
            let mut file = tempfile::NamedTempFile::new_in(parent)?;
            let data = serde_json::to_vec_pretty(settings)?;
            file.write_all(&data)?;
            file.as_file().sync_all()?;
            file.persist(&self.path).map_err(|error| error.error)?;
            Ok(())
        })();
        result.map_err(|error| format!("Could not save settings: {error}"))
    }
}

pub fn settings_path() -> Result<PathBuf, String> {
    if let Some(directory) = std::env::var_os("RUSTY_GPUI_SETTINGS_DIR") {
        if directory.is_empty() {
            return Err("RUSTY_GPUI_SETTINGS_DIR must not be empty.".into());
        }
        return Ok(PathBuf::from(directory).join("settings.json"));
    }
    #[cfg(target_os = "windows")]
    let directory = std::env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let directory = std::env::var_os("HOME")
        .map(|home| PathBuf::from(home).join("Library/Application Support"));
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let directory = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")));
    directory
        .map(|directory| directory.join("Rusty GPUI Settings/settings.json"))
        .ok_or_else(|| {
            "Could not find a user configuration directory. Set RUSTY_GPUI_SETTINGS_DIR.".into()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn store(directory: &tempfile::TempDir) -> Store {
        Store {
            path: directory.path().join("settings.json"),
        }
    }

    #[test]
    fn round_trip_replaces_existing_settings_including_unicode() {
        let directory = tempfile::tempdir().unwrap();
        let store = store(&directory);
        assert_eq!(store.load().unwrap(), None);
        store.save(&Settings::default()).unwrap();
        let updated = Settings {
            workspace: "Café 東京".into(),
            updates: true,
            dark_mode: true,
        };
        store.save(&updated).unwrap();
        assert_eq!(store.load().unwrap(), Some(updated));
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }

    #[test]
    fn corrupt_settings_are_reported_and_preserved() {
        let directory = tempfile::tempdir().unwrap();
        let store = store(&directory);
        fs::write(&store.path, b"{broken json").unwrap();
        assert!(store.load().is_err());
        assert_eq!(fs::read(&store.path).unwrap(), b"{broken json");
    }

    #[test]
    fn invalid_save_preserves_previous_file() {
        let directory = tempfile::tempdir().unwrap();
        let store = store(&directory);
        store.save(&Settings::default()).unwrap();
        let before = fs::read(&store.path).unwrap();
        let invalid = Settings {
            workspace: "   ".into(),
            ..Settings::default()
        };
        assert!(store.save(&invalid).is_err());
        assert_eq!(fs::read(&store.path).unwrap(), before);
    }

    #[test]
    fn failed_replacement_cleans_temp_file_and_allows_retry() {
        let directory = tempfile::tempdir().unwrap();
        let store = store(&directory);
        fs::create_dir(&store.path).unwrap();
        assert!(store.save(&Settings::default()).is_err());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
        fs::remove_dir(&store.path).unwrap();
        store.save(&Settings::default()).unwrap();
        assert_eq!(store.load().unwrap(), Some(Settings::default()));
    }

    #[test]
    fn validation_counts_characters_and_rejects_control_characters() {
        for name in ["", " ", "line\nbreak", "name\t", &"界".repeat(81)] {
            assert!(
                Settings {
                    workspace: name.into(),
                    ..Settings::default()
                }
                .validate()
                .is_err()
            );
        }
        assert!(
            Settings {
                workspace: "界".repeat(80),
                ..Settings::default()
            }
            .validate()
            .is_ok()
        );
    }
}
