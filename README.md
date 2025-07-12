# Sosal Client

Простой клиент для запуска локального Shadowsocks прокси на основе строки подключения в формате `ss://...`. Аналог Outline client на Rust

---

## Требования

- Установленный [`sslocal`](https://github.com/shadowsocks/shadowsocks-rust) — клиент Shadowsocks, который запускается из командной строки.
- Rust (для сборки программы) — [https://rustup.rs/](https://rustup.rs/)

---

## Установка sslocal

`sslocal` — это исполняемый файл клиента Shadowsocks.

```bash
cargo install shadowsocks-rust --bin sslocal 
```

## Как использовать

1. Склонируйте и соберите проект:

```bash
git clone https://github.com/wwweblo/sosal-vpn-client
cd sosal-vpn-client
```

2. Запустите программу:

```bash
cargo run
```

3. Введите строку подключения Shadowsocks формата `ss://...`, например:

```
ss://YWVzLTEyOC1nY206cGFzc3dvcmQ=@1.2.3.4:8388
```

4. Программа автоматически создаст конфигурационный файл `ss_config.json` и запустит `sslocal` с этим конфигом.

5. Локальный Socks5 прокси будет доступен по адресу `127.0.0.1:1080`.

---

## Структура конфигурационного файла

Конфигурация генерируется автоматически и сохраняется в `ss_config.json` в формате:

```json
{
  "server": "адрес_сервера",
  "server_port": порт,
  "password": "пароль",
  "method": "метод_шифрования",
  "local_address": "127.0.0.1",
  "local_port": 1080,
  "timeout": 300
}
```
