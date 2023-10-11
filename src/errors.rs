use thiserror::*;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("GPU not found. Are you drivers installed?")]
    NoAdapter,
}

#[derive(Debug, Error, Clone, Copy)]
pub enum PipelineError {
    #[error("You need to initialise a Pipeline before drawing to the screen.")]
    NotInitialised,
    #[error("The path for the shader source is not valid.")]
    InvalidPath,
}
