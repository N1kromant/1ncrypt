#![allow(non_snake_case)]

use std::fs::{File, OpenOptions};
use std::io::{self, empty, Read, Write};

pub fn readFile(filename: &str) -> io::Result<String> {
    // Открываем файл или создаем, если его нет
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    // Читаем содержимое файла
    let mut content = String::new();
    file.read_to_string(&mut content);

    return Ok(content);
}

pub fn clearFile(filename: &str) -> io::Result<()> {
    let mut file: File = File::create(filename)?;

    return Ok(());
}

pub fn writeFile(filename: &str, text: &str) -> io::Result<()> {
    let mut file: File = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open(filename)?;

    file.write(text.as_bytes())?;

    return Ok(());
}
