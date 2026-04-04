pub struct ConfigureWalletInput {
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}

pub struct ConfigureWalletOutput {
    pub wallet_id: String,
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}
