pub mod error;
pub mod bin_format;
pub mod csv_format;
pub mod txt_format;

pub use error::{ParserError, Result};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    #[serde(rename = "TX_ID")]
    pub tx_id: u64,
    
    #[serde(rename = "TX_TYPE")]
    pub tx_type: String,
    
    #[serde(rename = "FROM_USER_ID")]
    pub from_user_id: u64,
    
    #[serde(rename = "TO_USER_ID")]
    pub to_user_id: u64,
    
    #[serde(rename = "AMOUNT")]
    pub amount: f64,
    
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    
    #[serde(rename = "STATUS")]
    pub status: String,
    
    #[serde(rename = "DESCRIPTION")]
    pub description: String,
}