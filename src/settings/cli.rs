//! This module handles the command line interface.

use clap::{Args, Parser, ValueHint};
use serde_derive::Deserialize;

use crate::settings::{Codec, ScaleAlgorithm};

/// Available video options.
#[derive(Debug, Clone, Args, Deserialize)]
#[clap(next_help_heading = Some("VIDEO OPTIONS"))]
pub struct VideoOptions {
    /// Bitrate in kilobits per second.
    #[clap(short, long)]
    pub bitrate: Option<u32>,

    /// Frame rate in frames per second.
    #[clap(short, long)]
    pub fps: Option<u64>,

    /// Width of the output video.
    /// If not specified, the width of the first image is used.
    #[clap(long)]
    pub width: Option<u32>,

    /// Height of the output video.
    /// If not specified, the height of the first image is used.
    #[clap(long)]
    pub height: Option<u32>,

    /// Name of the video codec to use.
    #[clap(long, action)]
    pub ignore_aspect_ratio: bool,

    /// Name of the video codec to use.
    #[clap(short, long, value_enum)]
    pub codec: Option<Codec>,

    /// Name of the image scaling algorithm to use.
    #[clap(long, value_enum)]
    pub scaling_algorithm: Option<ScaleAlgorithm>,
}

/// Simple program to convert a sequence of images to a webm video.
#[derive(Debug, Clone, Parser, Deserialize)]
#[clap(author, version, about)]
pub struct Options {
    /// Path of the source directory.
    #[clap(value_hint = ValueHint::DirPath)]
    pub source_directory: String,

    /// Place the output into <output_file>.
    #[clap(short, value_hint = ValueHint::FilePath)]
    pub output_file: Option<String>,

    /// Use verbose output
    #[clap(short, long)]
    pub verbose: Option<String>,

    #[clap(flatten)]
    pub video_options: VideoOptions,
}

impl Options {
    /// Parse the command line arguments.
    pub fn new() -> Self {
        Options::parse()
    }
}
