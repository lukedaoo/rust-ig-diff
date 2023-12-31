use std::error::Error;
use std::io;
use std::process;

use csv::ReaderBuilder;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
    let mut first = String::new();
    input_file_path_from_console(&mut first);

    let mut second = String::new();
    input_file_path_from_console(&mut second);

    println!("----- Input -----");
    // Trim the input to remove any leading/trailing whitespaces or newlines
    let first = first.trim();
    let first_records = read_csv_file(first);
    print_records("First file".to_string(), &first_records);

    let second = second.trim();
    let second_records = read_csv_file(second);
    print_records("Second file".to_string(), &second_records);

    // compare
    println!("---- Report -----");
    diff(first_records.unwrap(), second_records.unwrap());
}

#[derive(Debug, Eq)]
struct Record {
    user_id: String,
    user_name: String,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Hash for Record {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
    }
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

fn print_records(s: String, result: &Result<Vec<Record>, Box<dyn Error>>) {
    match result {
        Ok(rows) => {
            println!("{}: {} records", s, rows.len());
        }
        Err(e) => {
            eprintln!("Error reading CSV file: {}", e);
            process::exit(1);
        }
    }
}

fn diff(record1: Vec<Record>, record2: Vec<Record>) {
    let len1 = record1.len();
    let len2 = record2.len();

    let set1: HashSet<Record> = record1.into_iter().collect();
    let set2: HashSet<Record> = record2.into_iter().collect();

    if len1 < len2 {
        let status = format!("{} {} following", "increase".to_string(), (len2 - len1));
        let difference = set2.difference(&set1).clone().collect();

        report(status, difference);
        return;
    }

    if len1 > len2 {
        let status = format!("{} {} following", "decrease".to_string(), (len1 - len2));
        let difference = set1.difference(&set2).clone().collect();

        report(status, difference);
        return;
    }

    let status = format!("Neutral");
    let diff1 = set1.difference(&set2).clone().collect();
    let diff2 = set2.difference(&set1).clone().collect();

    print_status(status);
    print_diff(diff1);
    print_diff(diff2);
}

fn report(status: String, diff: HashSet<&Record>) {
    print_status(status);
    print_diff(diff);
}

fn print_status(status: String) {
    println!("Status: {}", status);
}

fn print_diff(diff: HashSet<&Record>) {
    if diff.len() == 0 {
        return;
    }
    println!("Records:");
    for record in diff {
        println!("       {:?}", record);
    }
}
