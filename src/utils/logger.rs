use std::fs::*;
use std::io::{stdout, Write};
use chrono;

pub enum LogType {
    Info,
    Caution,
    Error,
    CriticalError
}

pub fn log(logtype: LogType, message: String) {
    let final_msg: String = craft_message(logtype, message);
    write_to_console(&final_msg);
    write_to_logfile(&final_msg);
}



// Private functions

fn craft_message(logtype: LogType, message: String) -> String{
    let date = chrono::offset::Local::now().format("%Y-%m-%d, %hh:%mm");
    let type_str: &str = match(logtype){
        LogType::Info => "Info",
        LogType::Caution => "Caution",
        LogType::Error => "ERROR",
        LogType::CriticalError => "CRITICAL ERROR"
    };

    format!("{date} - [{type_str}] - {message}")
}


fn write_to_console(message: &String) {
    let mut lock = stdout().lock();
    writeln!(lock, "{}", message).unwrap();
}

fn write_to_logfile(message: &String) {
    let path: String = std::env::var("LOG_FILE").expect("LOG_FILE must be set");;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

