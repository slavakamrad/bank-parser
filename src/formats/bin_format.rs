use crate::{ParserError, Result, Transaction};
use std::convert::TryInto;
use std::io::{Read, Write};

const MAGIC: [u8; 4] = *b"YPBN";

pub fn from_binary<R: Read>(mut reader: R) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let magic_positions: Vec<usize> = buffer
        .windows(4)
        .enumerate()
        .filter(|(_, window)| window == &MAGIC)
        .map(|(i, _)| i)
        .collect();

    for (_, &pos) in magic_positions.iter().enumerate() {
        let next_pos = if pos + 8 < buffer.len() {
            let payload_size =
                u32::from_be_bytes(buffer[pos + 4..pos + 8].try_into().unwrap()) as usize;
            pos + 8 + payload_size
        } else {
            buffer.len()
        };

        if next_pos > buffer.len() {
            continue;
        }

        let record = &buffer[pos..next_pos];

        if record.len() < 58 {
            continue;
        }

        let payload_size = u32::from_be_bytes(record[4..8].try_into().unwrap()) as usize;

        if payload_size < 42 || payload_size > 1000 {
            continue;
        }

        let expected_size = 8 + payload_size;
        if record.len() < expected_size {
            continue;
        }

        let tx_id = u64::from_be_bytes(record[8..16].try_into().unwrap());

        let tx_type = match record[16] {
            0 => "DEPOSIT",
            1 => "TRANSFER",
            2 => "WITHDRAWAL",
            _ => continue,
        };

        let from_user_id = u64::from_be_bytes(record[17..25].try_into().unwrap());
        let to_user_id = u64::from_be_bytes(record[25..33].try_into().unwrap());
        let amount = u64::from_be_bytes(record[33..41].try_into().unwrap());
        let timestamp = i64::from_be_bytes(record[41..49].try_into().unwrap());
        let status = match record[49] {
            0 => "SUCCESS",
            1 => "FAILURE",
            2 => "PENDING",
            _ => continue,
        };

        let desc_len = payload_size - 42;
        let desc_start = 50;
        let desc_end = desc_start + desc_len;

        if desc_end > record.len() {
            continue;
        }

        let description_bytes = &record[desc_start..desc_end];

        let description = if description_bytes.len() >= 4 {
            String::from_utf8_lossy(&description_bytes[4..])
                .trim_matches('\0')
                .to_string()
        } else {
            String::from_utf8_lossy(description_bytes)
                .trim_matches('\0')
                .to_string()
        };

    
        let transaction = Transaction {
            tx_id,
            tx_type: tx_type.to_string(),
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status: status.to_string(),
            description,
        };

        transactions.push(transaction);
    }

    Ok(transactions)
}

pub fn to_binary<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<()> {
    for tx in transactions {
        let desc_bytes = tx.description.as_bytes();
        let payload_size = 42 + 4 + desc_bytes.len();

        if payload_size > u32::MAX as usize {
            return Err(ParserError::BinaryFormat(format!(
                "Слишком большая запись: payload_size={} > {}",
                payload_size,
                u32::MAX
            )));
        }
        writer.write_all(&MAGIC)?;
        writer.write_all(&(payload_size as u32).to_be_bytes())?;
        writer.write_all(&tx.tx_id.to_be_bytes())?;

        let tx_type_code = match tx.tx_type.as_str() {
            "DEPOSIT" => 0,
            "TRANSFER" => 1,
            "WITHDRAWAL" => 2,
            _ => {
                return Err(ParserError::BinaryFormat(format!(
                    "Некорректный тип транзакции: {}",
                    tx.tx_type
                )));
            }
        };
        writer.write_all(&[tx_type_code])?;
        writer.write_all(&tx.from_user_id.to_be_bytes())?;
        writer.write_all(&tx.to_user_id.to_be_bytes())?;

        // amount - u64 в BIG-ENDIAN
        writer.write_all(&tx.amount.to_be_bytes())?;
        writer.write_all(&tx.timestamp.to_be_bytes())?;
        let status_code = match tx.status.as_str() {
            "SUCCESS" => 0,
            "FAILURE" => 1,
            "PENDING" => 2,
            _ => {
                return Err(ParserError::BinaryFormat(format!(
                    "Некорректный статус: {}",
                    tx.status
                )));
            }
        };
        writer.write_all(&[status_code])?;

        // 4 байта заголовка описания (длина описания + 1)
        let desc_len = desc_bytes.len() as u32 + 1;
        writer.write_all(&desc_len.to_le_bytes())?;
        writer.write_all(desc_bytes)?;
    }

    Ok(())
}
