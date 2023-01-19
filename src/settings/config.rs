//! This module handles the configuration file.

use serde_derive::Deserialize;
use std::fs;

use super::{Codec, ScaleAlgorithm, Settings, VideoSettings};

/// Available video settings.
#[derive(Debug, Clone, Deserialize)]
pub struct VideoConfig {
    /// Bitrate in kilobits per second.
    pub bitrate: Option<u32>,

    /// Frame rate in frames per second.
    pub fps: Option<u64>,

    /// Width of the output video.
    pub width: Option<u32>,

    /// Height of the output video.
    pub height: Option<u32>,

    /// Name of the video codec to use.
    pub ignore_aspect_ratio: Option<bool>,

    /// Name of the video codec to use.
    pub codec: Option<Codec>,

    /// Name of the image scaling algorithm to use.
    pub scaling_algorithm: Option<ScaleAlgorithm>,
}

/// Simple program to convert a sequence of images to a webm video.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Path of the source directory.
    pub source_directory: Option<String>,

    /// Place the output into <output_file>.
    pub output_file: Option<String>,

    /// Use verbose output
    pub verbose: Option<String>,

    pub video_settings: VideoConfig,
}

impl Config {
    pub fn new() -> Option<Self> {
        let home_key = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        let config_filepath = match std::env::var(home_key) {
            Ok(home) => format!("{}/.config/img-to-webm/config.toml", home),
            Err(_) => "./config.toml".to_owned(),
        };
        println!("config_filepath: {}", config_filepath);
        let config_file = match fs::read_to_string(config_filepath) {
            Ok(file) => file,
            Err(err) => {
                println!("Error: {}", err);
                debug!("Couldn't reading config file: {}", err);
                return None;
            }
        };
        let config: Config = match toml::from_str(&config_file) {
            Ok(config) => config,
            Err(err) => {
                debug!("Couldn't parse config file: {}", err);
                return None;
            }
        };
        debug!("Settings from config file: {:#?}", &config);
        Some(config)
    }

    pub fn into_settings(self) -> Settings {
        let default = Settings::default();
        Settings {
            source_directory: self.source_directory.unwrap_or(default.source_directory),
            output_file: self.output_file.unwrap_or(default.output_file),
            verbose: self.verbose.unwrap_or(default.verbose),
            video_settings: VideoSettings {
                bitrate: self
                    .video_settings
                    .bitrate
                    .unwrap_or(default.video_settings.bitrate),
                fps: self
                    .video_settings
                    .fps
                    .unwrap_or(default.video_settings.fps),
                width: self.video_settings.width,
                height: self.video_settings.height,
                ignore_aspect_ratio: self
                    .video_settings
                    .ignore_aspect_ratio
                    .unwrap_or(default.video_settings.ignore_aspect_ratio),
                codec: self
                    .video_settings
                    .codec
                    .unwrap_or(default.video_settings.codec),
                scaling_algorithm: self
                    .video_settings
                    .scaling_algorithm
                    .unwrap_or(default.video_settings.scaling_algorithm),
            },
        }
    }
}
