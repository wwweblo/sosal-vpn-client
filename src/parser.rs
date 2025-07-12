use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
pub struct ShadowsocksConfig {
    pub method: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

fn base64_decode_padded(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let mut s = input.to_string();
    while s.len() % 4 != 0 {
        s.push('=');
    }
    general_purpose::STANDARD.decode(&s)
}

pub fn parse_outline_url(ss_url: &str) -> Result<ShadowsocksConfig, String> {
    println!("🔍 Входная строка: {}", ss_url);

    let url_str = ss_url
        .strip_prefix("ss://")
        .ok_or("Строка не начинается с ss://")?;
    println!("🔧 После удаления префикса ss:// => {}", url_str);

    let (encoded, rest) = url_str
        .split_once('@')
        .ok_or("Не найдена часть '@' — строка должна содержать base64@host")?;

    println!("🧪 base64 часть: {}", encoded);
    println!("🌐 адресная часть: {}", rest);

    let decoded_bytes = base64_decode_padded(encoded)
        .map_err(|e| format!("Ошибка декодирования base64: {}", e))?;

    let decoded_str = String::from_utf8(decoded_bytes)
        .map_err(|e| format!("Ошибка декодирования UTF-8: {}", e))?;

    println!("🔓 Раскодировано: '{}'", decoded_str);

    if !decoded_str.contains(':') {
        return Err("❗️ В decoded строке отсутствует ':' — неверный формат".into());
    }

    let (method, password) = decoded_str
        .split_once(':')
        .ok_or("Не удалось разделить метод и пароль")?;

    println!("🔑 Метод: {}", method);
    println!("🔑 Пароль: {}", password);

    // Очищаем query и fragment (всё после ? или #)
    let mut address_cleaned = rest;
    if let Some((a, _)) = rest.split_once('?') {
        address_cleaned = a;
    } else if let Some((a, _)) = rest.split_once('#') {
        address_cleaned = a;
    }
    // Убираем конечный слэш, если есть
    address_cleaned = address_cleaned.strip_suffix('/').unwrap_or(address_cleaned);

    println!("📦 Очищенный адрес: {}", address_cleaned);

    let (host, port_str) = address_cleaned
        .split_once(':')
        .ok_or("Не удалось выделить хост и порт")?;

    let port = port_str
        .parse::<u16>()
        .map_err(|e| format!("Порт должен быть числом: {}", e))?;

    println!("📡 Хост: {}, Порт: {}", host, port);

    Ok(ShadowsocksConfig {
        method: method.to_string(),
        password: password.to_string(),
        server: host.to_string(),
        port,
    })
}
