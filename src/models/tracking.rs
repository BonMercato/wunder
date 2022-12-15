use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TrackingRequest {
    /// The code of a carrier. This information is mandatory for a registered carrier.
    pub carrier_code: Option<String>,
    /// The name of a carrier. This information is mandatory for an unregistered carrier.
    pub carrier_name: Option<String>,
    /// The tracking url of a carrier. This information is unused for registered carriers (because computed automatically). This information is optional for unregistered carriers.
    pub carrier_url: Option<String>,
    /// The carrier tracking number. This information is mandatory for a registered carrier with a URL requiring a tracking number.
    pub tracking_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct XmlTrackingRequest {
    /// The order id.
    pub order_id: String,
    /// The code of a carrier. This information is mandatory for a registered carrier.
    pub carrier_code: Option<String>,
    /// The name of a carrier. This information is mandatory for an unregistered carrier.
    pub carrier_name: Option<String>,
    /// The tracking url of a carrier. This information is unused for registered carriers (because computed automatically). This information is optional for unregistered carriers.
    pub carrier_url: Option<String>,
    /// The carrier tracking number. This information is mandatory for a registered carrier with a URL requiring a tracking number.
    pub tracking_number: Option<String>,
}

impl From<XmlTrackingRequest> for TrackingRequest {
    fn from(xml: XmlTrackingRequest) -> Self {
        Self {
            carrier_code: xml.carrier_code,
            carrier_name: xml.carrier_name,
            carrier_url: xml.carrier_url,
            tracking_number: xml.tracking_number,
        }
    }
}