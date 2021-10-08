use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config)->Result<(),Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}",contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

// fn parse_config(args: &[String]) -> (&str, &str) {
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config {query, filename}

//     // let query = &args[1];
//     // let filename = &args[2];
//     // (query, filename)

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enouogh arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";


        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            //do something with line
            results.push(line);
        }
    }

    results
}

