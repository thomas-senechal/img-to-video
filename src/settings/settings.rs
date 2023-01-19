//! This module contains the settings for the program.

use serde_derive::Deserialize;
use webm::mux;

use crate::settings::cli;

use super::config::Config;

/// Available image scaling algorithms.
/// This enum is used to parse the CLI argument.
/// See [image::imageops::FilterType] for more information.
#[derive(clap::ValueEnum, Debug, Clone, Deserialize)]
pub enum ScaleAlgorithm {
    /// Nearest Neighbor
    Nearest,
    /// Linear Filter
    Triangle,
    /// Cubic Filter
    CatmullRom,
    /// Gaussian Filter
    Gaussian,
    /// Lanczos with window 3
    Lanczos3,
}

/// Available video codecs.
/// This enum is used to parse the CLI argument.
#[derive(clap::ValueEnum, Debug, Clone, Deserialize)]
pub enum Codec {
    /// VP8
    Vp8,
    /// VP9
    Vp9,
}

/// Available video settings.
#[derive(Debug, Clone, Deserialize)]
pub struct VideoSettings {
    /// Bitrate in kilobits per second.
    pub bitrate: u32,

    /// Frame rate in frames per second.
    pub fps: u64,

    /// Width of the output video.
    pub width: Option<u32>,

    /// Height of the output video.
    pub height: Option<u32>,

    /// Name of the video codec to use.
    pub ignore_aspect_ratio: bool,

    /// Name of the video codec to use.
    pub codec: Codec,

    /// Name of the image scaling algorithm to use.
    pub scaling_algorithm: ScaleAlgorithm,
}

impl VideoSettings {
    /// Returns the video codec ID as a tuple.
    pub fn convert_codec(&self) -> (vpx_encode::VideoCodecId, mux::VideoCodecId) {
        match self.codec {
            Codec::Vp8 => (vpx_encode::VideoCodecId::VP8, mux::VideoCodecId::VP8),
            Codec::Vp9 => (vpx_encode::VideoCodecId::VP9, mux::VideoCodecId::VP9),
        }
    }

    /// Returns the image scaling algorithm for the image crate.
    pub fn convert_scaling_algorithm(&self) -> image::imageops::FilterType {
        match self.scaling_algorithm {
            ScaleAlgorithm::Nearest => image::imageops::FilterType::Nearest,
            ScaleAlgorithm::Triangle => image::imageops::FilterType::Triangle,
            ScaleAlgorithm::CatmullRom => image::imageops::FilterType::CatmullRom,
            ScaleAlgorithm::Gaussian => image::imageops::FilterType::Gaussian,
            ScaleAlgorithm::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        VideoSettings {
            bitrate: 25000,
            fps: 30,
            width: None,
            height: None,
            ignore_aspect_ratio: false,
            codec: Codec::Vp9,
            scaling_algorithm: ScaleAlgorithm::Nearest,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// Path of the source directory.
    pub source_directory: String,

    /// Place the output into <output_file>.
    pub output_file: String,

    /// Use verbose output
    pub verbose: String,

    pub video_settings: VideoSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            source_directory: ".".to_string(),
            output_file: "output.webm".to_string(),
            verbose: "WARN".to_string(),
            video_settings: VideoSettings::default(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        let default_config = match Config::new() {
            Some(c) => c.into_settings(),
            None => Settings::default(),
        };

        let cli_options = cli::Options::new();

        let settings = Settings {
            source_directory: cli_options.source_directory.clone(),
            output_file: cli_options
                .output_file
                .clone()
                .unwrap_or(default_config.output_file),
            verbose: cli_options
                .verbose
                .clone()
                .unwrap_or(default_config.verbose),
            video_settings: VideoSettings {
                bitrate: cli_options
                    .video_options
                    .bitrate
                    .unwrap_or(default_config.video_settings.bitrate),
                fps: cli_options
                    .video_options
                    .fps
                    .unwrap_or(default_config.video_settings.fps),
                width: cli_options.video_options.width,
                height: cli_options.video_options.height,
                ignore_aspect_ratio: cli_options.video_options.ignore_aspect_ratio,
                codec: cli_options
                    .video_options
                    .codec
                    .clone()
                    .unwrap_or(default_config.video_settings.codec),
                scaling_algorithm: cli_options
                    .video_options
                    .scaling_algorithm
                    .unwrap_or(default_config.video_settings.scaling_algorithm),
            },
        };
        settings.set_log_level();
        settings
    }

    /// Set the log level based on the verbose option.
    /// If the verbose option is empty, the log level is set to "ERROR".
    fn set_log_level(&self) {
        if !self.verbose.is_empty() && std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", &self.verbose);
        }
    }
}
