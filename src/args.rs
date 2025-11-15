
use std::{collections::HashMap};

use crate::utils::extract_filename;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output_dir: String,
    pub is_verbose: bool,
}


impl Args {

    pub fn parse() -> Args {
        let mut args = Args {
            input: String::new(),
            output_dir: String::from("./output"),
            is_verbose: false,
        };

        let mut env_args = std::env::args();

        env_args.next();

        let mut options_map: HashMap<String, String> = HashMap::new();
        let mut flags_map: HashMap<String, bool> = HashMap::new();

        let options = vec!["-i", "-o"];

        while let Some(arg) = env_args.next() {
            if options.contains(&arg.as_str()) {
                let option_value = match env_args.next() {
                    Some(value) => value,
                    None => String::new(),
                };
                options_map.insert(arg, option_value);
            } else {
                flags_map.insert(arg, true);
            }
        }

        if let Some(input_filename) = options_map.get("-i") {
            args.input = input_filename.to_string();
        }

        if let Some(output_dir) = options_map.get("-o") {
            args.output_dir = output_dir.to_string();
        } else if let Some(output_dir) = std::env::var("MD_NOTES_OUTDIR").ok() {
            args.output_dir = output_dir;
        }

        args.is_verbose = flags_map.contains_key("-v");

        args
    }

    pub fn output(&self) -> String {
        format!(
            "{}{}{}",
            self.output_dir,
            std::path::MAIN_SEPARATOR.to_string(),
            self.input_filename().replace(".md", "").replace(".txt", "")
        )
    }

    pub fn input_filename(&self) -> String {
        let (_, filename) = extract_filename(&self.input);
        filename
    }

}