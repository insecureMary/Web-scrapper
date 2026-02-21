extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::{Either, Html};

//a mini web scraper that fetches two pages and prints the title of the one that responds first
fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let winner = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("A page, {winner:?} returned first");
        match winner {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let response_text = trpl::get(url).await.text().await;
        Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
