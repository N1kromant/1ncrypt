#![allow(non_snake_case)]

use std::io;
mod MutateFileModule;
mod EncrytionModule;
fn main() {
    let outFileName = "f.wut";
    let keyFileName = "yek.key";

    let filedata = MutateFileModule::readFile(keyFileName).expect("NODATA");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    let input = input.trim();

    let filedataBin: String = filedata.as_bytes().iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let inputBin: String = input.as_bytes().iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("inputBin:{}", inputBin);
    println!("filedataBin:{}", filedataBin);

    let encrypted = EncrytionModule::blockEncrypt(input.as_bytes(), filedata.as_bytes());

    let encryptedBin = encrypted.iter()
    .map(|byte| byte.to_string())
    .collect::<Vec<String>>()
    .join(" ");

    println!("encoded:{}", encryptedBin);

    MutateFileModule::writeFile(outFileName, unsafe { &String::from_utf8_unchecked(encrypted)});

    io::stdin()
        .read_line(&mut String::new())
        .expect("Не удалось прочитать строку");
}
