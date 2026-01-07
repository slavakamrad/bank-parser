use crate::{ParserError, Result, Transaction};
use std::io::{Read, Write};

pub fn from_text<R: Read>(_reader: R) -> Result<Vec<Transaction>> {
    // TODO: Реализовать парсинг текстового формата
    Err(ParserError::TextFormat("Not implemented yet".to_string()))
}

pub fn to_text<W: Write>(_transactions: &[Transaction], _writer: W) -> Result<()> {
    // TODO: Реализовать запись в текстовый формат
    Err(ParserError::TextFormat("Not implemented yet".to_string()))
}
