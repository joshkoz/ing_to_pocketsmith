#[derive(thiserror::Error, Debug)]
pub enum Error {
    // For starting, remove as code matures...
    #[error("Generic: {0}")]
    Generic(String),

    // For starting, remove as code matures...
    #[error("Static: {0}")]
    Static(&'static str),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] toml::de::Error),

    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    DateTimeParse(#[from] chrono::format::ParseError),
}
