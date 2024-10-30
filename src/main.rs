#![allow(non_snake_case)]

use std::io;
mod MutateFileModule;
mod EncrytionModule;

fn main() {
    let outFileName = "f.wut";
    let keyFileName = "yek.key";

    let mut key = MutateFileModule::readFile(keyFileName).expect("NODATA");
    let mut input = String::new();

    let iv:Vec<u8> = vec![]; 

    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    let mut input = input.trim();

    let keyBin: String = key.as_bytes().to_vec().iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let inputBin: String = input.as_bytes().to_vec().iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let mut encrypted = EncrytionModule::encrypt(&mut input.as_bytes().to_vec(), &mut key.as_bytes().to_vec());

    let encryptedBin = encrypted.iter()
    .map(|byte| byte.to_string())
    .collect::<Vec<String>>()
    .join(" ");

    MutateFileModule::writeFile(outFileName, unsafe { &String::from_utf8_unchecked(encrypted)});

    io::stdin()
        .read_line(&mut String::new())
        .expect("Не удалось прочитать строку");
}
