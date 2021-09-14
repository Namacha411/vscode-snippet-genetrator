use std::io::{self, BufRead};
use std::path::Path;
use std::{fs::File, path::PathBuf};

use serde::Serialize;

pub struct Snippet {
    path: PathBuf,
}

impl Snippet {
    pub fn new(path: PathBuf) -> Snippet {
        Snippet { path }
    }

    pub fn make_json(&mut self) -> String {
        #[derive(Debug, Serialize)]
        pub struct Json {
            prefix: String,
            body: Vec<String>,
        }
        let prefix = self.path.file_name().unwrap().to_str().unwrap().to_string();
        let body = make_body(self.path.clone());
        let json = Json {
            prefix,
            body,
        };
        let json = serde_json::to_string(&json).unwrap();
        json
    }
}

fn make_body(path: PathBuf) -> Vec<String> {
    let lines = read_lines(path);
    let s = lines.filter_map(|s| s.ok()).collect::<Vec<String>>();
    s
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => {
            panic!("{:?}", why);
        }
    };
    io::BufReader::new(file).lines()
}
