
extern crate chrono;

use self::chrono::{ DateTime, Local, Timelike, Datelike };

use std::fs::{ File, OpenOptions };
use std::io::prelude::*;

pub struct Logger {
	file_name: String,
	file_handle: File,
}

pub fn new_log() -> Logger
{
	let local : DateTime<Local> = Local::now();
	let date = local.date();

	if let Err(e) = std::fs::create_dir("logs") {
		eprintln!("Failed to create directory! {}", e);
	}

	let formatted_name = format!("logs\\{}{}{}_{:02}{:02}{:02}_Log.txt",
							date.year().to_string(),
							date.month().to_string(),
							date.day().to_string(),
							local.hour().to_string(), 
							local.minute().to_string(), 
							local.second().to_string(), 
						).to_string();


	return Logger {
		file_name : String::from(formatted_name.clone()),
		file_handle : OpenOptions::new()
					.append(true)
					.create(true)
					.open(formatted_name.clone())
					.unwrap()
	};
}

impl Logger
{
	pub fn write_log(&mut self, msg : &str) {
		if let Err(e) = writeln!(self.file_handle, "{}", msg) {
			eprintln!("Failed to write to file: {}", e);
		}
	}
}