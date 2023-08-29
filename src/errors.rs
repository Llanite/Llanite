use thiserror::*;

#[derive(Debug, Error)]
pub enum BoosterError {
    #[error("No GPU detected. Do you have drivers?")]
    NoGPU,
    #[error("Creating a window failed. Is there a window system?")]
    WindowFailure,
    #[error("Creating a surface failed.")]
    SurfaceFailure,
    #[error("Resizing a window failed, is the window smaller than 0px?")]
    ResizeFailure
}
