use reqwest::Error;
use scraper::{Selector, Html, ElementRef};
use std::result::Result;
use reqwest::blocking;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Site {
    name: String,
    stories: Vec<Story>,
}

#[derive(Debug)]
struct Story {
    title: String,
    link: Option<String>,
}

impl Display for Site {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        for i in &self.stories {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl Display for Story {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.link {
            Some(link) => { write!(f, "\t{}\n\t\t({})", self.title, link) }
            None => writeln!(f, "\t\n{}", self.title),
        }
    }
}


pub async fn this_week_in_rust_org() -> Result<Site, Error> {
    let selector = Selector::parse("div.col-md-12 a").unwrap();
    let body = get_html("https://this-week-in-rust.org/blog/archives/index.html").await?;

    let stories = body.select(&selector).map(|element| Story { title: parse_title(element), link: parse_link(element) }).collect();
    let site = Site { name: "this-week-in-rust.org".to_string(), stories: stories };
    Ok(site)
}

async fn get_html(uri: &str) -> Result<Html, Error> {
    Ok(Html::parse_document(&blocking::get(uri)?.text()?))
}

fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let link_str = link_str.to_owned();
        link = Some(link_str);
    }
    link
}

fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}