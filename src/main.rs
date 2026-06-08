use romaji::RomajiExt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Ошибка: не указана входная строка");
        eprintln!("Использование: kana-converter \"ромадзи\"");
        std::process::exit(1);
    }
    
    let input = &args[1];
    
    // Конвертация в хирагану и катакану
    let hiragana = input.to_hiragana();
    let katakana = input.to_katakana();
    
    println!("{}", hiragana);
    println!("{}", katakana);
}
