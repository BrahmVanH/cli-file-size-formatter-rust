use std::env;
use std::num::ParseIntError;
use std::io::{ self, BufRead, Read, Error as IoError };

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
enum ErrorTypes {
    ParseIntError(ParseIntError),
    IoError(IoError),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String,
}

impl Sizes {
    fn new(bytes: u64) -> Self {
        Self {
            bytes: format!("{}", bytes),
            kilobytes: format!("{:.2}", (bytes as f64) / 1000.0),
            megabytes: format!("{:.2}", (bytes as f64) / 1_000_000.0),
            gigabytes: format!("{:.5}", (bytes as f64) / 1_000_000_000.0),
            terabytes: format!("{:.8}", (bytes as f64) / 1_000_000_000_000.0),
        }
    }
    fn print_self(&self) {
        println!("{:?}", self);
    }
}

fn extract_file_size_unit(size: &String) -> String {
    let file_size_unit = size
        .trim()
        .chars()
        .skip_while(|c| (c.is_numeric() || c.is_whitespace()))
        .take_while(|c| c.is_alphabetic())
        .collect();
    file_size_unit
}

fn extract_file_size_number(size: &String) -> String {
    let file_size_number = size
        .trim()
        .chars()
        .take_while(|c| c.is_numeric())
        .collect();
    file_size_number
}





fn format_size(size: &String) -> Result<Sizes, ParseIntError> {
    // pass in string that contains numbers and maybe a unit
    // parse u64 from size string

    let size_value_string = extract_file_size_number(size);
    let mut size_value = match size_value_string.parse::<u64>() {
        Ok(value) => value,
        // Return error option early on error
        Err(error) => {
            return Err(error);
        }
    };

    // collect unit from size string
    let unit = extract_file_size_unit(size);

    // Create empty var for sizes struct instance
    let formatted_sizes: Sizes;
    // If the user input only numbers, handle as bytes
    if unit.len() < 2 {
        formatted_sizes = Sizes::new(size_value);
        return Ok(formatted_sizes);
    }

    // convert file size back to bytes
    size_value = match unit.to_lowercase().as_str() {
        "kb" => size_value * 1024,
        "mb" => size_value * 1024 * 1024,
        "gb" => size_value * 1024 * 1024 * 1024,
        "tb" => size_value * 1024 * 1024 * 1024 * 1024,
        _ => size_value,
    };

    // factor bytes into all file sizes
    formatted_sizes = Sizes::new(size_value);

    // Return Result<Sizes>
    Ok(formatted_sizes)
}

fn main() -> Result<(), ErrorTypes> {
    let args: Vec<String> = env::args().collect();
    println!("input your file size, with or without unit. unit-less is treated as bytes.");
    let mut input_string = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    (match handle.read_line(&mut input_string) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(ErrorTypes::IoError(e));
        }
    })?;

    let sizes = format_size(&input_string);

    match sizes {
        Ok(sizes) => {
            sizes.print_self();
            Ok(())
        }
        Err(e) => {
            return Err(ErrorTypes::ParseIntError(e));
        }
    }
}
