use std::error::Error;

struct Articles{
    articles: Vec<Article>
}

struct Article{
    title: String,
    description: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response)
    todo!()
}

 fn main() {}
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=cde7cbd805a447c1899a27d8cb493501";
    let articles: Result<{Unknown},Box<dyn>> = get_articles(url);
}


