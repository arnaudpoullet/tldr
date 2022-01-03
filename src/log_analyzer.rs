use crate::error::LogError;
use prettytable::{cell, row, Table};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use timed::timed;

#[derive(Deserialize, Debug)]
struct Log {
    #[serde(rename = "type")]
    log_type: String,
}

struct LogType {
    nb_objects: u32,
    total_size: u32,
}

#[timed]
pub fn read_file_and_print_table(file: File) -> Result<(), LogError> {
    let mut reader = log_reader::BufReader::new(file);
    let mut buffer = String::new();
    let mut nb_lines = 0;
    let mut stat_map = HashMap::new();

    while let Some(line_result) = reader.read_line(&mut buffer) {
        nb_lines += 1;
        match line_result {
            Ok(line) => {
                let line_size = line.len();
                let message = serde_json::from_str::<Log>(line).map_err(|e| {
                    LogError::from(format!(
                        "Could not deserialize line {}. Reason: {}",
                        nb_lines, e
                    ))
                })?;
                let entry = stat_map.entry(message.log_type).or_insert(LogType {
                    nb_objects: 0,
                    total_size: 0,
                });
                entry.nb_objects += 1;
                entry.total_size += line_size as u32
            }
            Err(e) => {
                let err_msg = format!("Could not read line {}. Reason: {}", nb_lines, e);
                return Err(LogError::from(err_msg));
            }
        }
    }
    let mut table = Table::new();
    table.add_row(row!["TYPE", "NUMBER OF OBJECTS", "TOTAL SIZE"]);
    for (key, log_type) in stat_map {
        table.add_row(row![
            &key,
            &log_type.nb_objects.to_string(),
            &log_type.total_size.to_string()
        ]);
    }
    table.printstd();
    Ok(())
}

//https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
mod log_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn new(file: File) -> Self {
            let reader = io::BufReader::new(file);
            Self { reader }
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
