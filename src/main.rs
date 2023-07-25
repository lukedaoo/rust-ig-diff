use std::error::Error;
use std::io;
use std::process;

use csv::ReaderBuilder;
use std::collections::HashSet;

fn main() {
    let mut first = String::new();
    input_file_path_from_console(&mut first);

    let mut second = String::new();
    input_file_path_from_console(&mut second);

    // Trim the input to remove any leading/trailing whitespaces or newlines
    let first = first.trim();
    let first_records = read_csv_file(first);
    // print_records(first_records);

    let second = second.trim();
    let second_records = read_csv_file(second);
    // print_records(second_records);

    // compare
    diff(first_records.unwrap(), second_records.unwrap());
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Record {
    user_id: String,
    user_name: String,
}

fn input_file_path_from_console(input_path: &mut String) {
    println!("Enter the path to the CSV file:");
    io::stdin()
        .read_line(input_path)
        .expect("Failed to read input path.");
}

fn read_csv_file(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        let following_record = Record {
            user_id: row.get(0).unwrap().to_string(),
            user_name: row.get(1).unwrap().to_string(),
        };
        rows.push(following_record);
    }

    Ok(rows)
}

#[warn(dead_code)]
fn print_records(result: Result<Vec<Record>, Box<dyn Error>>) {
    // Read the CSV file and process its content
    match result {
        Ok(rows) => {
            println!("CSV content:");
            for row in rows {
                println!("{:?}", row);
            }
        }
        Err(e) => {
            eprintln!("Error reading CSV file: {}", e);
            process::exit(1);
        }
    }
}

fn diff(record1: Vec<Record>, record2: Vec<Record>) {
    let record1_len = record1.len();
    let record2_len = record2.len();

    let mut state = "Neutral".to_string();

    if record1_len < record2_len {
        state = "Increase following".to_string();
    }

    if record1_len > record2_len {
        state = "Decrease following".to_string();
    }

    let record1_user_ids: HashSet<Record> = record1.into_iter().collect();
    let record2_user_ids: HashSet<Record> = record2.into_iter().collect();

    let difference: HashSet<&Record> = record1_user_ids
        .difference(&record2_user_ids)
        .clone()
        .collect();

    println!("Status: {}", state);
    println!("Difference between set1 and set2: {:?}", difference);
}
