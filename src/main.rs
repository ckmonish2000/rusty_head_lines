use serde::{Deserialize, Serialize};
use ureq::{get};
use colored::*;

#[derive(Serialize, Deserialize,Debug)]
struct Article{
    title:String,
    url:String,
}

#[derive(Serialize, Deserialize,Debug)]
struct Articles{
    articles: Vec<Article>,
}


fn main() {
    let val =get_news().unwrap();
    
    for i in 0..val.articles.len() {
        println!("{}",val.articles[i].title.green());
        println!("{}",val.articles[i].url.red());
        println!("\n");
    }
   }




// function to get news
fn get_news() -> Result<Articles,ureq::Error> {
    let path ="https://newsapi.org/v2/top-headlines?country=us&apiKey=5b9d5eb78659499fa9b0e4d2fee0059a";
    
    let body: Articles = get(path)
    .call()?
    .into_json()?;

    Ok(body)
}