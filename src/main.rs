use trpl::{Either, Html}; 
        // enum Either<A, B> {
        //     Left(A),
        //     Right(B),
        // }
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
 
    // функция ..trpl::run.. как бы создает среду, которая управляет 
    // асинхронным кодом (main так делать не умеет, поскольку 
    // она является точкой входа в программу)
    trpl::run(async {
        let res_page_title_1 = page_title(&args[1]); // Здесь возвращён ленивый фьючерс, который без использования ничего 
        let res_page_title_2 = page_title(&args[2]); // делать не будет,  а если его использовать, то он вернёт кортеж 
//                                                                                                                                  ( &str, Option<String> )
        let (url, maybe_title) = 
            match trpl::race(res_page_title_1, res_page_title_2).await { // trpl::race, которая возвращает значение, указывающее,
                Either::Left(left) => left,             // какой из переданных ей фьючерсов завершится первым.                            
                Either::Right(right) => right,          
        };                                                                                                                 

        println!("URL: {url}");

        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
