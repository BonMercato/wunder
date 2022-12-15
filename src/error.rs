use miette::Diagnostic;

use crate::models::invoices::OrderDocumentResponse;

#[derive(thiserror::Error, Diagnostic, Debug)]
pub enum WunderError {
    #[error("Tracking file does not exist: {0}")]
    #[diagnostic(code(wunder::error::tracking_file_not_found))]
    TrackingFileNotFound(String),

    #[error("Invoice file does not exist: {0}")]
    #[diagnostic(code(wunder::error::invoice_file_not_found))]
    InvoiceFileNotFound(String),

    #[error("Document format not supported: {0}")]
    #[diagnostic(code(wunder::error::document_format_not_supported))]
    DocumentFormatNotSupported(String),

    #[error("Error while uploading document: {0:#?}")]
    #[diagnostic(code(wunder::error::document_upload_error))]
    DocumentUploadError(OrderDocumentResponse),

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

    #[error(transparent)]
    #[diagnostic(code(wunder::error::serde_json))]
    SerdeJson(#[from] serde_json::Error),
}