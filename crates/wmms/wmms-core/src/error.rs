
pub type Result<T> = std::result::Result<T, WMMSCoreError>;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum WMMSCoreError {
    #[error("Invalid canonical path: {0}")]
    InvalidPath(String),

    #[error("Id collision for {prefix}: '{a}' with '{b}'")]
    IdCollision { prefix: &'static str, a: String, b: String },

    #[error("Tick overflow")]
    TickOverflow,

    #[error("Numeric overflow")]
    NumericOverflow,

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}