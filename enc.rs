use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use rand::{distributions::Alphanumeric, Rng};

fn main() -> io::Result<()> {
    // Имя файла
    let filename = "f.wut";

    // Открываем файл или создаем, если его нет
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    // Читаем содержимое файла
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Если в файле меньше 16 символов, генерируем их
    if content.len() < 16 {
        let generated: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        
        // Записываем 16 сгенерированных символов в файл
        file.write_all(generated.as_bytes())?;
        println!("Generated and saved: {}", generated);
    } else {
        // Выводим первые 16 символов из файла
        let first_16 = &content[..16];
        println!("Read from file: {}", first_16);
    }

    Ok(())
}
