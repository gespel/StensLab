use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonSynthEntry {
    synth_type: String,
    frequency: f32
}

pub struct ScriptParser {
    path: String,
    files: Vec<String>,
}

impl ScriptParser {
    pub fn new(path: String) -> ScriptParser {
        let mut files: Vec<String> = Vec::new();
        let paths = fs::read_dir(&path).unwrap();
        for p in paths {
            let s = p.unwrap().path().to_str().unwrap().to_string();
            if PathBuf::from(&s).is_file() {
                files.push(s);
            }

        }
        ScriptParser {
            path,
            files,
        }
    }

    pub fn print_files(&self) {
        for f in &self.files {
            println!("{f}");
        }
    }

    pub fn parse_files(&self) {
        self.parse_setup_json();
    }

    pub fn parse_setup_json(&self) {
        let setup_path_string = format!("{}/setup.json", self.path);
        let setup_path = Path::new(setup_path_string.as_str());
        if setup_path.exists() {
            let ds: Value = serde_json::from_str(fs::read_to_string(setup_path).unwrap().as_str()).expect("Error while reading setup.json");
            println!("setup.json loaded!");
            for synth in ds["synths"].as_array().unwrap() {
                let s: JsonSynthEntry = serde_json::from_str(synth.to_string().as_str()).unwrap();
                println!("{}", s.synth_type);
                println!("{}", s.frequency);
            }
        }
        else {
            println!("setup.json not found!");
        }
    }
}