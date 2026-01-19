use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader};
use transact_parser::{ParserError, Result, Transaction, parse_format};

#[derive(Parser, Debug)]
#[command(name = "transact_compare")]
#[command(about = "Сравнивает транзакции из двух файлов разных форматов")]
#[command(version = "1.0")]
struct Cli {
    /// Первый файл для сравнения
    #[arg(long)]
    file1: String,

    /// Формат первого файла (csv, binary, text)
    #[arg(long = "format1")]
    format1: String,

    /// Второй файл для сравнения
    #[arg(long)]
    file2: String,

    /// Формат второго файла (csv, binary, text)
    #[arg(long = "format2")]
    format2: String,

    /// Выводить подробную информацию о различиях
    #[arg(short, long)]
    verbose: bool,
}

fn read_transactions(filename: &str, format: &str) -> Result<Vec<Transaction>> {
    let file = File::open(filename).map_err(|e| {
        ParserError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open {}: {}", filename, e),
        ))
    })?;

    let reader = BufReader::new(file);
    parse_format(reader, format)
}

fn normalize_description(desc: &str) -> String {
    // Нормализуем описание: убираем все кавычки и добавляем одну пару
    let clean_desc = desc.trim_matches('"').replace('"', "");
    if clean_desc.is_empty() {
        String::new()
    } else {
        format!("\"{}\"", clean_desc)
    }
}

fn compare_transactions(
    transactions1: &[Transaction],
    transactions2: &[Transaction],
    verbose: bool,
) -> bool {
    if transactions1.len() != transactions2.len() {
        println!(
            "❌ Разное количество транзакций: {} vs {}",
            transactions1.len(),
            transactions2.len()
        );
        return false;
    }

    if verbose {
        println!("📊 Сравнивается {} транзакций...", transactions1.len());
    }

    for (i, (t1, t2)) in transactions1.iter().zip(transactions2.iter()).enumerate() {
        let mut mismatches = Vec::new();

        if t1.tx_id != t2.tx_id {
            mismatches.push(format!("tx_id ({} vs {})", t1.tx_id, t2.tx_id));
        }
        if t1.tx_type != t2.tx_type {
            mismatches.push(format!("tx_type ({} vs {})", t1.tx_type, t2.tx_type));
        }
        if t1.from_user_id != t2.from_user_id {
            mismatches.push(format!(
                "from_user_id ({} vs {})",
                t1.from_user_id, t2.from_user_id
            ));
        }
        if t1.to_user_id != t2.to_user_id {
            mismatches.push(format!(
                "to_user_id ({} vs {})",
                t1.to_user_id, t2.to_user_id
            ));
        }
        if t1.amount != t2.amount {
            mismatches.push(format!("amount ({} vs {})", t1.amount, t2.amount));
        }
        if t1.timestamp != t2.timestamp {
            mismatches.push(format!("timestamp ({} vs {})", t1.timestamp, t2.timestamp));
        }
        if t1.status != t2.status {
            mismatches.push(format!("status ({} vs {})", t1.status, t2.status));
        }

        // Сравниваем нормализованные описания
        let desc1 = normalize_description(&t1.description);
        let desc2 = normalize_description(&t2.description);
        if desc1 != desc2 {
            mismatches.push(format!("description ({} vs {})", desc1, desc2));
        }

        if !mismatches.is_empty() {
            println!("❌ Несовпадение в транзакции #{}:", i + 1);
            for mismatch in mismatches {
                println!("   - {}", mismatch);
            }
            return false;
        }

        if verbose && i < 5 {
            println!("   ✓ Транзакция #{} совпадает", i + 1);
        }
    }

    true
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "📁 Чтение файла 1: '{}' (формат: {})",
        cli.file1, cli.format1
    );
    let transactions1 = read_transactions(&cli.file1, &cli.format1)?;

    println!(
        "📁 Чтение файла 2: '{}' (формат: {})",
        cli.file2, cli.format2
    );
    let transactions2 = read_transactions(&cli.file2, &cli.format2)?;

    println!("🔍 Начинаю сравнение...");

    if compare_transactions(&transactions1, &transactions2, cli.verbose) {
        println!("\n✅ Файлы идентичны!");
        println!("   Всего транзакций: {}", transactions1.len());
        Ok(())
    } else {
        println!("\n❌ Файлы различаются!");
        std::process::exit(1);
    }
}
