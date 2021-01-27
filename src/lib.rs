use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}



pub struct Config{
    pub query:String,
    pub filename:String
}
impl Config{
    pub fn new()->Config{
        Config{query:String::from(""),filename:String::from("")}
    }
    pub fn new_with_args(args:&[String])->Result<Config,MyError>{
        if args.len()>2{
            return Ok(Config{query:args[1].clone(),filename:args[2].clone()})
        }
        Err(MyError::new("Not enough arguments provided example: minigrep [word] [filename]"))
    }
}

pub fn run(config:Config)->Result<(),Box<Error>>{
    let mut f=File::open(&config.filename)?;
    let mut contents=String::new();
    f.read_to_string(&mut contents).expect("Error reading the file");
    for line in search_word(&contents[..], &config.query[..]){
        println!("Found reference of '{}' inside : '{}'",config.query,line)
    }
   Ok(())

}

pub fn parse_args(args:&[String])->Result<Config,MyError>{
    if args.len()<3{
        return Err(MyError::new("Not enough arguments provided example: minigrep [word] [filename]"))    
    }
    Ok(Config{query:args[1].clone(),filename:args[2].clone()})
}

# [cfg(test)]

mod test{
    use super::*;
    #[test]
    fn search_one_item_ok(){
        let query="andrea";
        let contents="andrea ciao bello come stai?
        Se stai bene
        andrea dimmi se stai bene";
        assert_eq!(vec!["andrea ciao bello come stai?","andrea dimmi se stai bene"],search_word(&contents, &query));
    }
}

fn search_word<'a>(contents:& 'a str,query:&str)->Vec<& 'a str>{
   let mut out_lines= Vec::new();
   for line in contents.lines(){
       if line.contains(query){
           out_lines.push(line.trim());
       }
   }
   out_lines

}
