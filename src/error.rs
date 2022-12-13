use miette::Diagnostic;

#[derive(thiserror::Error, Diagnostic, Debug)]
pub enum WunderError {
    #[error("Generic error: {0}")]
    #[diagnostic(code(wunder::error::generic))]
    Generic(String),

    #[error("Tracking file does not exist: {0}")]
    #[diagnostic(code(wunder::error::tracking_file_not_found))]
    TrackingFileNotFound(String),

    #[error(transparent)]
    #[diagnostic(code(wunder::error::io))]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(wunder::error::reqwest))]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    #[diagnostic(code(wunder::error::quick_xml))]
    QuickXml(#[from] quick_xml::DeError),

    #[error(transparent)]
    #[diagnostic(code(wunder::error::config))]
    Config(#[from] config::ConfigError),
}