pub mod error;
pub mod formats;

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

pub fn parse_format<R: std::io::Read>(reader: R, format: &str) -> Result<Vec<Transaction>> {
    match format.to_lowercase().as_str() {
        "csv" => formats::from_csv(reader),
        "binary" => formats::from_binary(reader),
        "text" => formats::from_text(reader),
        _ => Err(ParserError::UnsupportedFormat(format.to_string())),
    }
}

pub fn write_format<W: std::io::Write>(
    transactions: &[Transaction],
    writer: W,
    format: &str,
) -> Result<()> {
    match format.to_lowercase().as_str() {
        "csv" => formats::to_csv(transactions, writer),
        "binary" => formats::to_binary(transactions, writer),
        "text" => formats::to_text(transactions, writer),
        _ => Err(ParserError::UnsupportedFormat(format.to_string())),
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            tx_id: 0,
            tx_type: String::new(),
            from_user_id: 0,
            to_user_id: 0,
            amount: 0.0,
            timestamp: 0,
            status: String::new(),
            description: String::new(),
        }
    }
}
