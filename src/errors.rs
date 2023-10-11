use thiserror::*;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("GPU not found. Are you drivers installed?")]
    NoAdapter,
    #[error("Size too small")]
    ResizeTooSmall,
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("You need to initialise a Pipeline before drawing to the screen.")]
    NotInitialised,
    #[error("The path for the shader source is not valid.")]
    InvalidPath,
}
