use thiserror::*;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("GPU not found. Are you drivers installed?")]
    NoAdapter,
    #[error("Size too small")]
    ResizeTooSmall,
}
