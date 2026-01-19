# Financial Data Parser

Библиотека и CLI утилиты для работы с финансовыми данными в различных форматах.

## Что это?

Проект состоит из трёх частей:

1. **transact_parser** - библиотека для парсинга финансовых данных
2. **transact_converter** - утилита для конвертации между форматами
3. **transact_compare** - утилита для сравнения файлов

## Поддерживаемые форматы:

- CSV
- Binary
- Text

## Примеры работы с программой:

#### Чтение и конвертация файлов
```bash
cargo run --bin transact_converter -- \
  --input data/records_example.bin \
  --input-format binary \
  --output-format csv \
  --output test_binary.csv
```

#### Сравнение файлов
```bash
cargo run --bin transact_compare -- \
  --file1 data/records_example.bin \
  --format1 binary \
  --file2 data/records_example.csv \
  --format2 csv
```