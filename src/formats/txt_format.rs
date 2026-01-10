use crate::{ParserError, Result, Transaction};
use std::io::{BufRead, Read, Write};

pub fn from_text<R: Read>(reader: R) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let mut current_tx: Option<Transaction> = None;

    let buf_reader = std::io::BufReader::new(reader);

    for line in buf_reader.lines() {
        let line = line.map_err(ParserError::Io)?;
        let trimmed = line.trim();

        // Пустая строка = конец текущей транзакции
        if trimmed.is_empty() {
            if let Some(tx) = current_tx.take() {
                transactions.push(tx);
            }
            continue;
        }

        // Игнорируем строки, начинающиеся с '#'
        if trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            if current_tx.is_none() {
                current_tx = Some(Transaction::default());
            }

            if let Some(ref mut tx) = current_tx {
                apply_field(tx, key, value)?;
            }
        }
    }

    if let Some(tx) = current_tx.take() {
        transactions.push(tx);
    }

    Ok(transactions)
}

fn apply_field(tx: &mut Transaction, key: &str, value: &str) -> Result<()> {
    match key {
        "TX_ID" => {
            tx.tx_id = value
                .parse()
                .map_err(|_| ParserError::TextFormat(format!("Invalid TX_ID: {}", value)))?;
        }
        "TX_TYPE" => tx.tx_type = value.to_string(),
        "FROM_USER_ID" => {
            tx.from_user_id = value
                .parse()
                .map_err(|_| ParserError::TextFormat(format!("Invalid FROM_USER_ID: {}", value)))?;
        }
        "TO_USER_ID" => {
            tx.to_user_id = value
                .parse()
                .map_err(|_| ParserError::TextFormat(format!("Invalid TO_USER_ID: {}", value)))?;
        }
        "AMOUNT" => {
            tx.amount = value
                .parse()
                .map_err(|_| ParserError::TextFormat(format!("Invalid AMOUNT: {}", value)))?;
        }
        "TIMESTAMP" => {
            tx.timestamp = value
                .parse()
                .map_err(|_| ParserError::TextFormat(format!("Invalid TIMESTAMP: {}", value)))?;
        }
        "STATUS" => tx.status = value.to_string(),
        "DESCRIPTION" => tx.description = value.to_string(),
        _ => {}
    }
    Ok(())
}

pub fn to_text<W: Write>(transactions: &[Transaction], writer: W) -> Result<()> {
    let mut wtr = std::io::BufWriter::new(writer);

    for (i, tx) in transactions.iter().enumerate() {
        writeln!(wtr, "# Record {} ({})", i + 1, tx.tx_type)?;
        writeln!(wtr, "TX_TYPE: {}", tx.tx_type)?;
        writeln!(wtr, "TO_USER_ID: {}", tx.to_user_id)?;
        writeln!(wtr, "FROM_USER_ID: {}", tx.from_user_id)?;
        writeln!(wtr, "TIMESTAMP: {}", tx.timestamp)?;
        writeln!(wtr, "DESCRIPTION: \"{}\"", tx.description)?;
        writeln!(wtr, "TX_ID: {}", tx.tx_id)?;
        writeln!(wtr, "AMOUNT: {}", tx.amount)?;
        writeln!(wtr, "STATUS: {}", tx.status)?;
        writeln!(wtr)?;
    }

    wtr.flush()?;
    Ok(())
}
