
use regex::{Regex, Captures};
use crate::stamp::Stamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    hash: String,
    name: String,
    price_from: i32,
    price_to: i32,
    price_diff: i32,
    stamp: Stamp,
}

fn str_clean(s: String) -> String {
    let new_str = s.replace("&quot;", "").replace("\u{a0}", "").trim().to_string();
    new_str
}

impl Product {
    fn get_price(page_body: &String) -> (i32, i32, i32) {
        fn _get_val(caps: &Captures) -> i32 {
            let splited: Vec<&str> = caps.get(0).unwrap().as_str().split(">").collect();
            let price_string = str_clean(splited[1].to_string());
            price_string.parse::<i32>().unwrap_or(0)
        }

        let re = Regex::new(r#"class="price__value"\s?[^</]+"#).unwrap();
        let caps_iter: Vec<Captures> = re.captures_iter(page_body).collect();
        let price_from = _get_val(&caps_iter[0]);
        let price_to = _get_val(&caps_iter[1]);
        let price_diff = price_to - price_from;

        ( price_from, price_to, price_diff )
    }

    fn get_name(page_body: &String) -> String {
        let re = Regex::new(r#"<h1 class="title__main"\s?[^</]+"#).unwrap();
        let caps = re.captures(page_body).unwrap();
        let splited: Vec<&str> = caps.get(0).unwrap().as_str().split(">").collect();
        str_clean(splited[1].to_string())
    }

    fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn new(page_body: &String) -> Self {
        let (price_from, price_to, price_diff) = Self::get_price(page_body);
        let name = Self::get_name(page_body);
        let digest = md5::compute(&name);
        let p = Product {
            name: name,
            hash: format!("{:x}", digest),
            price_from,
            price_to,
            price_diff,
            stamp: Stamp::new(),
        };
        println!("JSON: {}", p.get_json());
        p
    }
}
