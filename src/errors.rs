use crate::TileSet;

/// This error type wraps other crate's errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Server request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Server returned HTTP error code: {0}")]
    Http(reqwest::StatusCode),
}

/// This error type represents an error from a request argument that was invalid
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ArgumentError {
    #[error("Zoom level {0} is too large for the tilset {1} (max: {2})")]
    ZoomTooLarge(u32, TileSet, u32),

    #[error("Zoom level {0} is too small for the tilset {1} (min: {2})")]
    ZoomTooSmall(u32, TileSet, u32),

    #[error("X coordinate {0} is too large for the zoom level {1} (max X: {2})")]
    XTooLarge(u32, u32, u32),

    #[error("Y coordinate {0} is too large for the zoom level {1} (max Y: {2})")]
    YTooLarge(u32, u32, u32),
}
