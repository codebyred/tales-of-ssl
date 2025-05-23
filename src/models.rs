use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredData {
    pub domain: String,
    pub serial_number: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Problem {
    pub private_key: String,
    pub required_data: RequiredData,
}

#[derive(Debug, Serialize)]
pub struct Solution {
    pub certificate: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Feedback {
    Success {
        success: bool,
        message: Option<String>,
    },
    Rejected {
        rejected: String,
    },
}


