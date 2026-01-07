use crate::{ParserError, Result, Transaction};
use std::io::{Read, Write};

pub fn from_csv<R: Read>(reader: R) -> Result<Vec<Transaction>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut transactions = Vec::new();

    for result in rdr.deserialize() {
        let record: Transaction = result.map_err(|e| ParserError::Csv(e))?;
        transactions.push(record);
    }

    Ok(transactions)
}

pub fn to_csv<W: Write>(transactions: &[Transaction], writer: W) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(writer);

    for transaction in transactions {
        wtr.serialize(transaction)?;
    }

    wtr.flush()?;
    Ok(())
}
