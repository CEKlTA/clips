extern crate json;
extern crate rand;
extern crate regex;

use regex::Regex;
use std::{
    env::args,
    error, fmt, fs,
    io::{self, prelude::*, Error, ErrorKind, Read, SeekFrom},
    iter::Iterator,
};

type AnyError = Box<dyn error::Error>;

struct VecMatchTuples {
    data: Vec<(String, String)>,
}

impl VecMatchTuples {
    fn new() -> Self {
        VecMatchTuples { data: Vec::new() }
    }
}

impl fmt::Display for VecMatchTuples {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.data {
            writeln!(f, "{}: {}", key, value)?
        }

        Ok(())
    }
}

struct ClipsData {
    file: fs::File,
    json: json::JsonValue,
}

impl ClipsData {
    fn new(path: &str) -> Result<Self, Error> {
        const DEFAULT_JSON: &str = "{}";

        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let mut file_buffer: String = String::new();
        file.read_to_string(&mut file_buffer)?;

        if file_buffer.is_empty() {
            file_buffer = String::from("{}");
        }

        let json = json::parse(&file_buffer).unwrap_or(json::parse(DEFAULT_JSON).unwrap());

        Ok(ClipsData { file, json })
    }

    fn find(&self, query: &str) -> Result<VecMatchTuples, AnyError> {
        let regex = Regex::new(query)?;

        let mut matches: VecMatchTuples = VecMatchTuples::new();

        self.json.entries().for_each(|(key, value)| {
            if let Some(v) = regex.find(key) {
                if v.len() == key.len() {
                    matches.data.push((key.to_string(), value.dump()));
                }
            }
        });

        if matches.data.is_empty() {
            return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "No entries found.",
            )));
        }

        Ok(matches)
    }

    fn modify(&mut self, key: &str, payload: &str) -> Result<String, AnyError> {
        let entries = match self.find(key) {
            Ok(vec) => {
                println!(
                    "key or keys already exists, do you want to {}? [Y]es|[n]o.",
                    match payload {
                        "-r" => "remove",
                        _ => "overwrite",
                    }
                );
                ask_user()?;
                vec
            }
            Err(_) => {
                let mut res = VecMatchTuples::new();
                res.data.push((key.to_string(), String::new()));
                res
            }
        };

        for entry in entries.data.into_iter() {
            match payload {
                "-g" => {
                    self.json[&entry.0] = json::parse(
                        generate_password(
                            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
                            16,
                        )
                        .as_str(),
                    )?
                }
                "-r" => {
                    self.json.remove(&entry.0);
                }
                value => self.json[&entry.0] = json::parse(value)?,
            };
        }

        let new_buffer = &self.json.dump();

        self.file.set_len(0).unwrap();
        self.file.seek(SeekFrom::Start(0))?;
        let _ = self.file.write(new_buffer.as_bytes())?;

        Ok(format!(
            "Data {}",
            match payload {
                "-r" => "removed",
                _ => "written",
            }
        ))
    }
}

fn main() -> Result<(), AnyError> {
    struct Messages;

    impl Messages {
        const USAGE: &'static str ="
\x1b[1;34mUsage:\x1b[0m
    \x1b[1;32mclips [target] [JSON | -r: remove | -g: generate]\x1b[0m

\x1b[1;34mCommands:\x1b[0m
    \x1b[1;32m[target]\x1b[0m     \x1b[0;32mThe key to search for or modify in the JSON file.\x1b[0m
    \x1b[1;32m[JSON]\x1b[0m       \x1b[0;32mThe JSON string to be added or modified.\x1b[0m
    \x1b[1;32m-r\x1b[0m           \x1b[0;32mRemove the specified key from the JSON file.\x1b[0m
    \x1b[1;32m-g\x1b[0m           \x1b[0;32mGenerate a random password and store it under the specified key.\x1b[0m

\x1b[1;34mExamples:\x1b[0m
    \x1b[0;32mclips key_name '{\"example\": \"data\"}'\x1b[0m
    \x1b[0;32mclips key_name -r\x1b[0m
    \x1b[0;32mclips key_name -g\x1b[0m
    \x1b[0;32mclips key_name\x1b[0m

\x1b[1;34mDescription:\x1b[0m
    \x1b[0;32mClips (CLI Password Storage) is a command-line tool designed for managing local JSON format data, ideal for password management. It allows you to search (with support for regex expressions), modify, remove, and generate data entries effortlessly.\x1b[0m

    \x1b[0;32mRunning clips without arguments will display this usage information.\x1b[0m
";
        const INVALID: &'static str =
            "\x1b[93mInvalid arguments. Run without args to see the correct usage.\x1b[0m";
    }

    let args: Vec<String> = args().collect();

    // Use for apendding args because replit lacks...
    /*
    {
        let debug_args = "";

        for arg in debug_args.split_ascii_whitespace() {
            args.push(String::from(arg));
        }
    }
    */

    let path = args[0].replace("clips.exe", "clips-data.json");

    let mut file: ClipsData = ClipsData::new(&path)?;

    match args.len() {
        1 => {
            println!("{}", Messages::USAGE);
        }
        2 => {
            println!("{}", file.find(&args[1])?);
        }
        3 => {
            println!("{}", file.modify(&args[1], &args[2])?);
        }
        _ => {
            println!("{}", Messages::INVALID);
        }
    }

    Ok(())
}

fn generate_password(chars: &str, length: u8) -> String {
    let mut password = String::new();

    for _ in 0..length {
        let random_number: usize = rand::random::<usize>() % (chars.len() - 1);
        let chosen_char = chars.chars().nth(random_number).unwrap();
        password.push(chosen_char);
    }

    password = format!(r#""{}""#, password);

    password
}

fn ask_user() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();

        match input.trim() {
            "y" | "" => break Ok(()),
            "n" => break Err(Error::new(ErrorKind::Interrupted, "Operation canceled.")),
            _ => println!("Invalid input. Please enter 'y' or nothing to continue, otherwise use 'n' to cancel the operation."),
        }
    }
}
