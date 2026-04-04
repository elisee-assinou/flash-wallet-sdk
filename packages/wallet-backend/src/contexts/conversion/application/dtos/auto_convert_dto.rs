/// DTO INPUT — données reçues du handler
pub struct AutoConvertInput {
    pub amount_sats: u64,
    pub momo_number: String,
    pub convert_ratio: f64,
}

/// DTO OUTPUT — données retournées au handler
pub struct AutoConvertOutput {
    pub transaction_id: String,
    pub amount_sats: u64,
    pub amount_xof: u64,
    pub status: String,
}
