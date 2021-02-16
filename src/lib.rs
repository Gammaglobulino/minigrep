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

pub enum SearchCase{
    Sensitive,
    Insensitve,
}


pub struct Config{
    pub query:String,
    pub filename:String,
    pub search_case:SearchCase
}
impl Config{
    pub fn new()->Config{
        Config{query:String::from(""),filename:String::from(""),search_case:SearchCase::Sensitive}
    }
    pub fn new_with_args(mut args:std::env::Args)->Result<Config,MyError>{
        args.next(); //skip app name
        let query = match args.next(){
            Some(arg) => arg,
            None => String::from("")
        };
        let filename=match args.next(){
            Some(arg) => arg,
            None => String::from("")
        };
        match args.next(){
                Some(arg) =>{
                    if arg=="sensitive"{
                        return Ok(Config{query,filename,search_case:SearchCase::Sensitive})
                     } 
                    else if arg =="insensitive"{return Ok(Config{query,filename,search_case:SearchCase::Insensitve})}

                }
                None => return Ok(Config{query,filename,search_case:SearchCase::Sensitive})
            
        }
        Err(MyError::new("Not enough arguments provided example: minigrep [word] [filename]"))
    }
    pub fn set_sensitive(& mut self){
        self.search_case=SearchCase::Sensitive;
    }
    pub fn set_insensitive(& mut self){
        self.search_case=SearchCase::Insensitve;
    }
}

pub fn run(config:Config)->Result<(),Box<Error>>{
    let mut f=File::open(&config.filename)?;
    let mut contents=String::new();
    f.read_to_string(&mut contents).expect("Error reading the file");
    match config.search_case{
        SearchCase::Sensitive =>{
            for line in search_word(&contents[..], &config.query[..]){
                println!("Found reference of '{}' inside : '{}'",config.query,line)
            }

        },
        SearchCase::Insensitve =>{
            for line in search_word_insensitive(&contents[..], &config.query[..]){
                println!("Found reference of '{}' inside : '{}'",config.query,line)
            }

        }

    }    
   Ok(())

}

pub fn parse_args(args:&[String])->Result<Config,MyError>{
    if args.len()>3{
        match &args[3][..]{
            "sensitive" => return Ok(Config{query:args[1].clone(),filename:args[2].clone(),search_case:SearchCase::Sensitive}),
            "insensitive" => return Ok(Config{query:args[1].clone(),filename:args[2].clone(),search_case:SearchCase::Insensitve}),
            &_ => return Ok(Config{query:args[1].clone(),filename:args[2].clone(),search_case:SearchCase::Sensitive})

        }
    
    }
    Err(MyError::new("Not enough arguments provided example: minigrep [word] [filename]"))
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
    #[test]
    fn search_case_sensitive(){
        let query="AndreA";
        let contents="andrea ciao bello come stai?
        Se stai bene
        AndreA dimmi se stai bene";
        assert_eq!(vec!["AndreA dimmi se stai bene"],search_word(&contents, &query));
    }
    #[test]
    fn search_case_insensitive(){
        let query="AndreA";
        let contents="andrea ciao bello come stai?
        Se stai bene
        andrea dimmi se stai bene";
        assert_eq!(vec!["andrea ciao bello come stai?","andrea dimmi se stai bene"],search_word_insensitive(&contents, &query));
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
fn search_word_insensitive<'a>(contents:& 'a str,query:&str)->Vec<& 'a str>{
    let mut out_lines= Vec::new();
    let insensitive_q=query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&insensitive_q){
            out_lines.push(line.trim());
        }
    }
    out_lines
 
 }
