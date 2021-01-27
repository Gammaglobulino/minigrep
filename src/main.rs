extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;


fn main(){
   let in_args:Vec<String>=env::args().collect();
   let config=Config::new_with_args(&in_args).unwrap_or_else(|err|{
       println!("Error parsing the in arguments: {}",err);
       process::exit(1);
   });
      
   println!("You are looking for the word: > {:?}",config.query);
   println!("inside the filename: > {:?}",config.filename);

   minigrep::run(config).unwrap_or_else(|err|{
    println!("Error opening the file: {}",err);
    process::exit(1);
});
   
}




