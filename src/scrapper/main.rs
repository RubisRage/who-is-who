use std::{fmt::Display, time::Duration};

use reqwest::blocking::Client;
use scraper::{Html, Selector};

enum ScrapError {
    Request(reqwest::Error),
    Retry,
    NameNotPresent,
    ParseName(String),
}

impl Display for ScrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrapError::Retry => write!(f, "Retried too many times!"),
            ScrapError::Request(e) => {
                write!(f, "{e}")
            }
            ScrapError::NameNotPresent => {
                write!(f, "Tag <span class=\"texto\"> containing professors name was not present!")
            }
            ScrapError::ParseName(text) => {
                write!(f, "Failed to parse '{text}', name did not contain ','!")
            }
        }
    }
}

impl From<reqwest::Error> for ScrapError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

struct ScrappedProfessor {
    name: String,
    picture: Option<String>,
}

impl ScrappedProfessor {
    fn scrap(
        url: &str,
        max_retries: u32,
        timeout: Duration,
    ) -> Result<Self, ScrapError> {
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
            .ok_or(ScrapError::NameNotPresent)?
            .inner_html();

        let (surname, firstname) = name_text
            .split_once(",")
            .ok_or_else(|| ScrapError::ParseName(name_text.clone()))?;

        let name = format!("{} {}", firstname.trim(), surname.trim());

        let picture = document
            .select(&img_selector)
            .filter_map(|img| {
                img.attr("src")
                    .filter(|src| src.starts_with("fotos"))
                    .filter(|src| *src != "fotos/logo.gif")
                    .map(From::from)
            })
            .next();

        Ok(ScrappedProfessor { name, picture })
    }
}

fn main() {
    let max_retries = 5;
    let timeout = Duration::from_secs(5);

    for i in 1..=100 {
        let url =
            format!("https://www.dis.ulpgc.es/profesorado/ficha.asp?id={i}");

        print!("Page({i}): ");

        match ScrappedProfessor::scrap(&url, max_retries, timeout) {
            Ok(ScrappedProfessor {
                name,
                picture: Some(picture),
            }) => {
                println!("Professor has name: '{name}' and photo: '{picture}'")
            }

            Ok(ScrappedProfessor { name, picture: _ }) => {
                println!("Professor has name: '{name}' and no photo")
            }

            Err(e) => println!("{e}")
        }
    }
}
