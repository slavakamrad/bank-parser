use crate::{ParserError, Result, Transaction};
use std::io::{Read, Write};

pub fn from_binary<R: Read>(_reader: R) -> Result<Vec<Transaction>> {
    // TODO: Реализовать парсинг бинарного формата
    Err(ParserError::BinaryFormat("Not implemented yet".to_string()))
}

pub fn to_binary<W: Write>(_transactions: &[Transaction], _writer: W) -> Result<()> {
    // TODO: Реализовать запись в бинарный формат
    Err(ParserError::BinaryFormat("Not implemented yet".to_string()))
}
