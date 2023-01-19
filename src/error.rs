use thiserror::Error as ErrorDerive;

#[derive(ErrorDerive, Debug)]
pub enum Error {
    #[error("IO error: `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Encoder error: `{0}`")]
    Encoder(#[from] vpx_encode::Error),

    #[error("Encoder error: `{0}`")]
    EncoderCustom(String),

    #[error("No images found in: `{0}`")]
    NoImages(String),
}
