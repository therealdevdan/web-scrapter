use trpl::{Either, Html}; 
use std::env;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let responce_text: String = trpl::get(url).await.text().await;
    let title: Option<String> = Html::parse(&responce_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}

fn main() {
    let args: Vec<String> = env::args().collect();
 
    trpl::run(async {
        let res_page_title_1 = page_title(&args[1]);  
        let res_page_title_2 = page_title(&args[2]);  
                                                                                                                                  ( &str, Option<String> )
        let (url, maybe_title) = 
            match trpl::race(res_page_title_1, res_page_title_2).await {
                Either::Left(left) => left,                                        
                Either::Right(right) => right,          
        };                                                                                                                 

        println!("URL: {url}");

        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
