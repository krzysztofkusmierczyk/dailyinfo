use std::io;

use chrono::NaiveDate;
use derive_more::From;
use scraper::{Html, Selector};

pub struct CalendarDay {
    pub name_days: Vec<String>,
    pub festivities: Vec<String>,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Request(ureq::Error),
    Io(io::Error),
}

pub fn get_calendar_day(at: NaiveDate) -> Result<CalendarDay> {
    // Download html
    let date_str = at.format("%Y-%m-%d").to_string();
    let form: Vec<(&str, &str)> = vec![("data", date_str.as_str())];
    let response = ureq::post("https://www.kalbi.pl/search").send_form(&form)?;
    let html: String = response.into_string()?;
    let document = Html::parse_document(&html);

    // Parse names
    let name_selector =
        Selector::parse(".calCard-name-day a").expect("Invalid selector for name day.");

    let names: Vec<String> = document
        .select(&name_selector)
        .map(|el| el.text().collect())
        .collect();

    // Parse festivities
    let main_festivity_selector =
        Selector::parse(".calCard-fete a").expect("Invalid selector for main festivity ");

    let main_festivities: Vec<String> = document
        .select(&main_festivity_selector)
        .map(|el| el.text().collect())
        .map(|s: String| s.trim().to_string())
        .collect();

    let other_festivities_selector =
        Selector::parse(".calCard-ententa a").expect("Invalid selector for festivities");

    let other_fesitvities: Vec<String> = document
        .select(&other_festivities_selector)
        .map(|el| el.text().collect())
        .map(|s: String| s.trim().to_string())
        .collect();

    let all_festivities: Vec<String> = main_festivities
        .into_iter()
        .chain(other_fesitvities.into_iter())
        .collect();

    Ok(CalendarDay {
        name_days: names,
        festivities: all_festivities,
    })
}
