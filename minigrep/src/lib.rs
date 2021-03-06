use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config: Config)->Result<(),Box<dyn Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
        // println!("With text:\n{}",contents);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a qurey string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        // if args.len() < 3 {
        //     return Err("Not enouogh arguments");
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename,case_sensitive })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape.";


        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)

        );
    }



}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
   //let mut results = Vec::new();

    //for line in contents.lines(){
    //    if line.contains(query){
    //        //do something with line
    //        results.push(line);
    //    }
    //}

    //results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Not String slice, but String
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}







