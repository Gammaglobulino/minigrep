extern crate minigrep;
use std::env::args;
use std::process;
use minigrep::Config;
/// *********************
/// * minigrep
/// * search for a world inside a file
/// * use: should intantiate a Config class
/// * Example
/// *
/// * # minigrep::run(config)
/// * return an Option

fn main(){
   
   let config=Config::new_with_args(std::env::args()).unwrap_or_else(|err|{
       eprintln!("Error parsing the in arguments: {}",err);
       process::exit(1);
   });
      
   println!("You are looking for the word: > {:?}",config.query);
   println!("inside the filename: > {:?}",config.filename);

   minigrep::run(config).unwrap_or_else(|err|{
    eprintln!("Error opening the file: {}",err);
    process::exit(1);
});
   
}




