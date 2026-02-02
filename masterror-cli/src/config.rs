// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Configuration management for masterror-cli.
//!
//! Supports layered configuration (highest priority first):
//! 1. CLI arguments
//! 2. Environment variables
//! 3. Local project config (.masterror.toml in current directory)
//! 4. Global config (~/.config/masterror/config.toml)
//! 5. Default values

use std::{collections::HashMap, env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

/// Global config file name.
const CONFIG_FILE: &str = "config.toml";

/// Local project config file name.
const LOCAL_CONFIG_FILE: &str = ".masterror.toml";

/// Config directory name.
const CONFIG_DIR: &str = "masterror";

/// Application configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// General settings.
    pub general: GeneralConfig,
    /// Display settings.
    pub display: DisplayConfig,
    /// Command aliases.
    pub aliases: HashMap<String, String>
}

/// General configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Language code (en, ru, ko).
    pub lang:    String,
    /// Enable colored output.
    pub colored: bool
}

/// Display section toggles.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
    /// Show translated error message.
    pub translation: bool,
    /// Show "why this happens" explanation.
    pub why:         bool,
    /// Show fix suggestions.
    pub fix:         bool,
    /// Show documentation links.
    pub links:       bool,
    /// Show original compiler output.
    pub original:    bool
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            lang:    "en".to_string(),
            colored: true
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            translation: true,
            why:         true,
            fix:         true,
            links:       true,
            original:    false
        }
    }
}

impl Config {
    /// Get global config directory path.
    pub fn dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(CONFIG_DIR))
    }

    /// Get global config file path.
    pub fn path() -> Option<PathBuf> {
        Self::dir().map(|p| p.join(CONFIG_FILE))
    }

    /// Get local project config path (.masterror.toml in current directory).
    pub fn local_path() -> Option<PathBuf> {
        env::current_dir().ok().map(|p| p.join(LOCAL_CONFIG_FILE))
    }

    /// Check if this is the first run (no global config exists).
    pub fn is_first_run() -> bool {
        Self::path().is_none_or(|p| !p.exists())
    }

    /// Load config with layered priority:
    /// 1. Local .masterror.toml (if exists)
    /// 2. Global ~/.config/masterror/config.toml
    /// 3. Defaults
    pub fn load() -> Result<Self> {
        // Try local config first
        if let Some(local_path) = Self::local_path()
            && local_path.exists()
        {
            let content = fs::read_to_string(&local_path)?;
            let config: Config = toml::from_str(&content).map_err(|e| AppError::ConfigParse {
                path:    local_path.clone(),
                message: e.message().to_string()
            })?;
            return Ok(config);
        }

        // Try global config
        let Some(path) = Self::path() else {
            return Ok(Self::default());
        };

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content).map_err(|e| AppError::ConfigParse {
            path:    path.clone(),
            message: e.message().to_string()
        })?;

        Ok(config)
    }

    /// Save config to global file.
    pub fn save(&self) -> Result<()> {
        let Some(dir) = Self::dir() else {
            return Err(AppError::Io(std::io::Error::other(
                "could not determine config directory"
            )));
        };

        fs::create_dir_all(&dir)?;

        let path = dir.join(CONFIG_FILE);
        let content = toml::to_string_pretty(self).map_err(|e| AppError::ConfigParse {
            path:    path.clone(),
            message: e.to_string()
        })?;

        fs::write(&path, content)?;
        Ok(())
    }

    /// Save config to local project file (.masterror.toml).
    pub fn save_local(&self) -> Result<PathBuf> {
        let path = env::current_dir()?.join(LOCAL_CONFIG_FILE);
        let content = toml::to_string_pretty(self).map_err(|e| AppError::ConfigParse {
            path:    path.clone(),
            message: e.to_string()
        })?;

        fs::write(&path, &content)?;
        Ok(path)
    }

    /// Resolve command alias.
    #[allow(dead_code)]
    pub fn resolve_alias<'a>(&'a self, cmd: &'a str) -> &'a str {
        self.aliases.get(cmd).map(String::as_str).unwrap_or(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.lang, "en");
        assert!(config.general.colored);
        assert!(config.display.why);
        assert!(config.display.fix);
    }

    #[test]
    fn test_toml_roundtrip() {
        let config = Config::default();
        let toml = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml).unwrap();
        assert_eq!(parsed.general.lang, config.general.lang);
    }
}
