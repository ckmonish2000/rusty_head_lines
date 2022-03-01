use serde::{Deserialize, Serialize};
use ureq::{get};

#[derive(Serialize, Deserialize,Debug)]
struct Article{
    // name:String,
    title:String,
}

#[derive(Serialize, Deserialize,Debug)]
struct Articles{
    articles: Vec<Article>,
}


fn main() {
    get_news().unwrap();
   }




// function to get news
fn get_news() -> Result<(),ureq::Error> {
    let path ="https://newsapi.org/v2/top-headlines?country=us&apiKey=5b9d5eb78659499fa9b0e4d2fee0059a";
    
    let body: Articles = get(path)
    .call()?
    .into_json()?;

    

    // let body2 = serde_json::from_str(body);

    println!("{:#?}",body);

    Ok(())
}