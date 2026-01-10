use alloc::string::String;
use wmms_core::error::WMMSCoreError;

pub type AspectResult<t> = core::result::Result<t, AspectError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AspectError {
    #[error(transparent)]
    Core(#[from] WMMSCoreError),

    #[error("invalid aspect path: {0}")]
    InvalidPath(String),

    #[error("unknown aspect: {0}")]
    UnknownAspect(String),

    #[error("registry is sealed")]
    Sealed,

    #[error("duplicate aspect path: {0}")]
    Duplicate(String),
}
