use std::io;

fn main() {
    println!("Type in your github user:");

    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Failed to  read user.");

    let response = reqwest::blocking::get( format!("https://github.com/{}?tab=repositories", user))
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("li>div>div>h3>a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
