use std::{path::PathBuf, fs};
use base64::Engine;
use base64::engine::general_purpose;
use clap::Parser;
use arguments::{ProgramArguments, ProgramMode};

pub mod arguments;

fn main() {
    let arguments = ProgramArguments::parse();

    match arguments.command {
        Some(ProgramMode::Encode { file_path }) => encode(file_path),
        Some(ProgramMode::Decode { text }) => decode(text),
        None => println!("No subcommand passed. Valid subcommands are encode or decode"),
    }
}

fn encode(file_path: PathBuf) {
    let file_content = fs::read(file_path).expect("Specified file not found");
    let encoded = base64::engine::general_purpose::GeneralPurpose::encode(&general_purpose::STANDARD, &file_content);
    println!("Encoded File:\n{}", encoded);
}

fn decode(text: String) {
    let decoded = base64::engine::general_purpose::GeneralPurpose::decode(&general_purpose::STANDARD, &text).unwrap();
    match String::from_utf8(decoded) {
        Ok(v) => println!("Decoded Text:\n{}", v),
        Err(e) => panic!("Invalid UTF-8 sequence passed as argument: {}", e),
    };
}
