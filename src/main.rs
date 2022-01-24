#![windows_subsystem = "windows"]

use base64::encode;
use std::fs;

fn main() {
    let input = fs::read("raw.bin").unwrap();
    let mut output:Vec<u8> = Vec::new();
    let key:u8 =3;

    for mut data in input {
        data=data ^ key;
        output.push(data);
    }

    let output=encode(output);
    fs::write("1.txt", output).unwrap();
}

