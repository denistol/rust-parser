mod stamp;
mod product;
use product::{Product};
use std::fs;
use std::{thread, time::Duration};

fn fetch_product(url: &str) -> Product {
    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();
    body.to_string();
    Product::new(&body)
}
fn get_list() -> Vec<String> {
    let contents = fs::read_to_string("./src/list.txt")
        .expect("Something went wrong reading the file");
    let vec = contents.lines().map(|el| String::from(el.trim())).collect::<Vec<String>>();
    vec
}
fn run(urls: Vec<String>) {
    let mut handles: Vec<_> = Vec::new();
    for i in urls {
        handles.push(
            thread::spawn(move || {
                fetch_product(&i);
            })
        )
    };
    println!("Threads: {}", handles.len());

    for i in handles {
        i.join().unwrap();
    };
}

fn main() {
    loop {
        let list = get_list();
        run(list);
        thread::sleep(Duration::from_secs(10));
    }
}