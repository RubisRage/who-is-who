use std::time::Duration;

use reqwest::blocking::Client;
use scraper::{Html, Selector};

enum ScrapError {
    Request(reqwest::Error),
    Retry,
    NotPresent(Kind),
    ParseName(String),
}

enum Kind {
    Name,
    Photo,
}

impl From<reqwest::Error> for ScrapError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

fn obtain_professor(
    url: &str,
    max_retries: u32,
    timeout: Duration,
) -> Result<(String, String), ScrapError> {
    let mut retries = 0;
    let res = loop {
        match Client::new().get(url).timeout(timeout).send() {
            Ok(res) => break Ok(res),
            Err(e) => {
                if e.is_timeout() && retries >= max_retries {
                    break Err(ScrapError::Retry);
                } else if !e.is_timeout() {
                    break Err(e.into());
                }
            }
        };

        retries += 1;
    }?;

    let document = Html::parse_document(&res.error_for_status()?.text()?);

    let name_selector = Selector::parse("span.texto")
        .expect("span.texto should be a valid CSS selector");
    let img_selector =
        Selector::parse("img").expect("img should be a valid CSS selector");

    let name_text = document
        .select(&name_selector)
        .next()
        .ok_or(ScrapError::NotPresent(Kind::Name))?
        .inner_html();

    let (surname, firstname) = name_text
        .split_once(",")
        .map(|(surname, firstname)| (surname.trim(), firstname.trim()))
        .ok_or_else(|| ScrapError::ParseName(name_text.clone()))?;

    let name = format!("{firstname} {surname}");

    let photo = document
        .select(&img_selector)
        .filter_map(|img| img.attr("src").filter(|src| src.starts_with("foto")))
        .next()
        .ok_or(ScrapError::NotPresent(Kind::Photo))?;

    Ok((name, photo.into()))
}

fn main() {
    let max_retries = 5;
    let timeout = Duration::from_secs(5);

    for i in 1..=100 {
        let url =
            format!("https://www.dis.ulpgc.es/profesorado/ficha.asp?id={i}");

        print!("Page({i}): ");
        match obtain_professor(&url, max_retries, timeout) {
            Ok((name, photo)) => {
                println!("Professor has name: '{name}' and photo: '{photo}'")
            }
            Err(ScrapError::Request(e)) => {
                println!("{e}");
            }
            Err(ScrapError::Retry) => println!("Retried too many times"),
            Err(ScrapError::NotPresent(Kind::Name)) => {
                println!("Name was not present")
            }
            Err(ScrapError::NotPresent(Kind::Photo)) => {
                println!("Photo was not present")
            }
            Err(ScrapError::ParseName(text)) => {
                println!("Failed to parse {text}")
            }
        }
    }
}
