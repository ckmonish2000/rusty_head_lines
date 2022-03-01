use colored::*;
use ureq::{get};
fn main() {
    get_news();
   }



// function to get news
fn get_news() -> Result<(),ureq::Error> {
    let path ="https://newsapi.org/v2/top-headlines?country=us&apiKey=5b9d5eb78659499fa9b0e4d2fee0059a";
    
    let body = get(path)
    .call()?
    .into_string()?;

    println!("{}",body);

    Ok(())
}