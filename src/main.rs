mod convert;
mod error;
mod images;
mod settings;

use error::Error;
use images::get_images;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};
use std::path::Path;
use webm::mux;
use webm::mux::Track;

use crate::settings::Settings;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    env_logger::init();
    let settings: Settings = Settings::new();
    info!("Settings: {:#?}", &settings);

    if let Err(err) = img_to_webm(settings) {
        let _ = writeln!(io::stderr(), "{}", err);
        error!("{}", err);
        std::process::exit(1);
    }
}

fn img_to_webm(settings: settings::Settings) -> Result<()> {
    let src_path: &Path = Path::new(&settings.source_directory);
    let dst_filename: &Path = Path::new(&settings.output_file);
    info!("Reading directory: {}", src_path.display());
    let images = get_images(&src_path)?;
    info!("Got {} images", images.len());
    if images.is_empty() {
        let error = format!("No images found in {}", src_path.display());
        error!("{}", error);
        return Err(Error::NoImages(src_path.display().to_string()));
    }

    let width = match settings.video_settings.width {
        Some(w) => w,
        None => images[0].width(),
    };
    let height = match settings.video_settings.height {
        Some(h) => h,
        None => images[0].height(),
    };

    let out = match {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(dst_filename)
    } {
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };

    let mut webm =
        mux::Segment::new(mux::Writer::new(out)).expect("Could not initialize the multiplexer.");
    let mut vt = webm.add_video_track(
        width,
        height,
        None,
        settings.video_settings.convert_codec().1,
    );

    let mut vpx = vpx_encode::Encoder::new(vpx_encode::Config {
        width,
        height,
        timebase: [1, 1000],
        bitrate: settings.video_settings.bitrate,
        codec: settings.video_settings.convert_codec().0,
    })
    .map_err(|err| {
        let error_msg = format!("Could not initialize the VPX encoder: {}", err);
        Error::EncoderCustom(error_msg)
    })?;

    info!("Start encoding images...");
    for (index, i) in images.iter().enumerate() {
        info!(
            "Encoding images {:.1}%",
            index as f32 / images.len() as f32 * 100.0
        );
        let resized_img = match width != i.width() || height != i.height() {
            true => match settings.video_settings.ignore_aspect_ratio {
                false => i.resize(
                    width,
                    height,
                    settings.video_settings.convert_scaling_algorithm(),
                ),
                true => i.resize_exact(
                    width,
                    height,
                    settings.video_settings.convert_scaling_algorithm(),
                ),
            },
            false => i.clone(),
        };
        let frame = resized_img.clone().into_rgb8();
        let yuv = convert::convert_rgb_to_yuv420(
            resized_img.width(),
            resized_img.height(),
            &frame,
            image::ColorType::Rgb8.bytes_per_pixel().into(),
        );
        let ms = 1_000 / settings.video_settings.fps * index as u64;

        for frame in vpx.encode(ms as i64, &yuv).map_err(Error::Encoder)? {
            vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
        }
    }

    info!("Finished encoding images.");

    info!("Start writing webm...");
    let mut frames = vpx.finish().map_err(Error::Encoder)?;
    while let Some(frame) = frames.next().map_err(Error::Encoder)? {
        vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
    }

    let _ = webm.finalize(None);
    info!("Finished writing webm.");
    Ok(())
}
