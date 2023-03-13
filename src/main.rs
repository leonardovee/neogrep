use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let mut neo_grep = NeoGrep::new(args[1].clone(), Path::new(&args[2]).to_owned());
    neo_grep.run();
}

struct NeoGrep {
    query: String,
    path: PathBuf,
}

impl NeoGrep {
    fn new(query: String, path: PathBuf) -> Self {
        NeoGrep { query, path }
    }

    fn run(&mut self) {
        self.search_folders(self.path.clone());
    }

    fn search_folders(&mut self, path: PathBuf) {
        if !path.is_dir() {
            return;
        }
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            if let Some(cur_path) = path.unwrap().path().to_str() {
                let new_path = Path::new(cur_path).to_owned();
                if new_path.is_dir() {
                    self.search_folders(new_path);
                    continue;
                }
                let file = fs::read_to_string(cur_path);
                if let Ok(contents) = file {
                    self.query_file(contents, new_path.to_str().unwrap())
                }
                self.search_folders(new_path);
            }
        }
    }

    fn query_file(&mut self, contents: String, file: &str) {
        let mut found: Vec<String> = vec![];
        for (line_num, line) in contents.lines().enumerate() {
            if line.contains(&self.query.clone()) {
                found.push(format!("Line: {}; {}", line_num, line).to_string());
            }
        }
        if found.is_empty() {
            return;
        }
        println!("{}", file);
        for line in found {
            println!("{}", line);
        }
        println!();
    }
}
