use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct OrderDocuments {
    pub order_documents: Vec<OrderDocument>,
}

#[derive(Debug, Serialize)]
pub struct OrderDocument {
    pub file_name: String,
    pub type_code: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderDocumentResponse {
    pub errors_count: Option<i32>,
    pub order_documents: Vec<OrderDocumentErrors>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDocumentErrors {
    pub errors: Vec<OrderDocumentError>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDocumentError {
    pub code: String,
    pub message: String,
    pub field: String,
}