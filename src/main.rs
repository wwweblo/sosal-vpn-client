use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;
use std::process::Command;

mod parser;
use parser::{parse_outline_url, ShadowsocksConfig};

fn write_config_file(config: &ShadowsocksConfig, path: &Path) {
    let json = serde_json::json!({
        "server": config.server,
        "server_port": config.port,
        "password": config.password,
        "method": config.method,
        "local_address": "127.0.0.1",
        "local_port": 1080,
        "timeout": 300
    });

    let mut file = File::create(path).expect("❌ Не удалось создать конфигурационный файл");
    file.write_all(json.to_string().as_bytes())
        .expect("❌ Не удалось записать в конфигурационный файл");
}

fn main() {
    println!("🧷 Введите строку подключения (ss://...):");

    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).expect("❌ Ошибка чтения ввода");
    let input = input.trim();

    match parse_outline_url(input) {
        Ok(config) => {
            println!("✅ Парсинг прошёл успешно:");
            println!("  Сервер: {}:{}", config.server, config.port);
            println!("  Метод: {}", config.method);

            let config_path = Path::new("ss_config.json");
            write_config_file(&config, config_path);

            println!("🚀 Запуск sslocal на порту 1080...");

            let status = Command::new("sslocal")
                .args(&["-c", config_path.to_str().unwrap()])
                .status()
                .expect("❌ Не удалось запустить sslocal");

            if !status.success() {
                eprintln!("❌ sslocal завершился с ошибкой.");
            }
        }
        Err(e) => {
            eprintln!("❌ Ошибка разбора строки подключения: {}", e);
        }
    }
}
