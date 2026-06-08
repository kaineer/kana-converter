use romaji::RomajiExt;
use std::collections::HashMap;

// Встраиваем YAML словарь прямо в бинарник
const DICT_YAML: &str = include_str!("../data/reading_to_kanji.yaml");

type ReadingToKanji = HashMap<String, Vec<String>>;

struct Dictionary {
    by_reading: ReadingToKanji,
}

impl Dictionary {
    /// Загрузка словаря из встроенных данных
    pub fn from_embedded() -> anyhow::Result<Self> {
        let by_reading: ReadingToKanji = serde_yaml::from_str(DICT_YAML)?;
        Ok(Dictionary { by_reading })
    }

    /// Поиск точного совпадения чтения
    pub fn search_exact(&self, reading: &str) -> Vec<&String> {
        self.by_reading
            .get(reading)
            .map(|kanjis| kanjis.iter().collect())
            .unwrap_or_default()
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Ошибка: не указана входная строка");
        eprintln!("Использование: kana-converter <ромадзи>");
        std::process::exit(1);
    }

    let input = &args[1];

    // Конвертируем ромадзи в хирагану
    let hiragana = input.to_hiragana();

    // Выводим первую строку: хирагана
    println!("{}", hiragana);

    // Выводим вторую строку: катакана
    println!("{}", input.to_katakana());

    // Загружаем встроенный словарь и ищем кандзи по чтению
    match Dictionary::from_embedded() {
        Ok(dict) => {
            for kanji in dict.search_exact(&hiragana) {
                println!("{}", kanji);
            }
        }
        Err(_e) => {
            // Молча игнорируем ошибки словаря, но для отладки можно раскомментировать:
            // eprintln!("Ошибка загрузки словаря: {}", e);
        }
    }

    Ok(())
}
