extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::str;

const VERBOSE: &str = "verbose";
const TEMPLATE: &str = "template";
const REPLACEMENTS: &str = "replacements";
const OUTPUT: &str = "output";

fn main() {
    let matches = App::new("replace")
        .version("1.0")
        .author("zsiegel")
        .about("a simple template engine")
        .arg(Arg::with_name(VERBOSE)
            .short("v")
            .help("Sets to verbose output"))
        .arg(Arg::with_name(TEMPLATE)
            .short("t")
            .help("Sets the template file to be evaluated")
            .takes_value(true))
        .arg(Arg::with_name(OUTPUT)
            .short("o")
            .long(OUTPUT)
            .help("Sets the output file")
            .takes_value(true))
        .arg(Arg::with_name(REPLACEMENTS)
            .short("r")
            .help("Sets the replacement variables")
            .takes_value(true)
            .multiple(true))
        .get_matches();

    let is_verbose = match matches.occurrences_of(VERBOSE) {
        0 => false,
        _ => true,
    };

    let filename = matches.value_of(TEMPLATE).unwrap();

    if is_verbose {
        println!("Reading template file: {}", filename);
    }
    let mut file = match File::open(filename) {
        Err(reason) => {
            eprintln!("Unable to open file: {}", reason.to_string());
            process::exit(1);
        }
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(reason) => {
            eprintln!("Unable to read file: {}", reason.to_string());
            process::exit(1);
        }
        Ok(_) => {}
    }

    //Find all substitutions needed in the format {{ VAR_X }}
    let regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    let mut substitutions: Vec<String> = Vec::new();
    for token in regex.captures_iter(contents.as_str()) {
        substitutions.push(token[0].replace("{{", "")
            .replace("}}", "")
            .trim().to_string());
    }

    match substitutions.len() {
        0 => println!("No substitutions needed"),
        _ => {
            if is_verbose {
                println!("This template requires the following substitutions");
                for sub in &substitutions {
                    println!(" - {}", sub);
                }
            }

            //Capture the available replacements
            let available_values: Vec<&str> = match matches.values_of(REPLACEMENTS) {
                Some(values) => values.collect(),
                None => Vec::new(),
            };

            let mut values = HashMap::new();
            for replacement in &available_values {
                let kv: Vec<&str> = replacement.splitn(2, "=").collect();
                values.insert(kv[0], kv[1]);
            }

            let mut missing_values: Vec<&str> = Vec::new();
            for sub_needed in &substitutions {
                if !values.contains_key(sub_needed.as_str()) {
                    missing_values.push(sub_needed);
                }
            }

            if missing_values.len() > 0 {
                eprintln!("\nMissing variables needed to complete template");
                for missing in missing_values {
                    eprintln!(" - {}", missing);
                }
                process::exit(1);
            }

            //Build the template
            for substitution in &substitutions {
                let to_replace = format!("{{{{ {} }}}}", substitution);
                let replacement = values.get(substitution.as_str()).unwrap();
                contents = contents.replace(to_replace.as_str(),
                                            replacement);
            }

            let path = match matches.value_of(OUTPUT) {
                Some(file) => {
                    Path::new(file)
                }
                None => {
                    println!("{}", contents);
                    process::exit(0);
                }
            };

            let mut new_file = match File::create(&path) {
                Err(reason) => {
                    eprintln!("Unable to create file: {}", reason.to_string());
                    process::exit(1);
                }
                Ok(file) => file,
            };

            match new_file.write_all(contents.as_bytes()) {
                Err(reason) => {
                    eprintln!("Unable to write to file: {}", reason.to_string());
                    process::exit(1);
                }
                Ok(_) => {
                    println!("successfully wrote to {:?}", path)
                }
            }
        }
    }
}
