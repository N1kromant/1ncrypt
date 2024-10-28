#![allow(non_snake_case)]

use std::io;
mod MutateFileModule;
// mod Encrypter;
fn main() {
    let getFileName = "f.wut";
    let setFileName = "yek.key";

    let filedata = MutateFileModule::readFile(getFileName).expect("NODATA");
    
    println!("Data is: \"{}\"", filedata);
    MutateFileModule::clearFile(setFileName);
    // FileManager::writeFile(setFileName, "flag{}");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Не удалось прочитать строку");
}
