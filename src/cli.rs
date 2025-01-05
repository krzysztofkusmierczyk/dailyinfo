use chrono::NaiveDate;
use clap::Parser;
#[derive(Debug, Parser, PartialEq)]
#[command(name = "dailyinfo")]
pub struct Cli {
    #[arg(value_parser=parse_date,long)]
    pub calendar_date: Option<NaiveDate>,
}

fn parse_date(arg: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(arg, "%Y-%m-%d")
}

impl Cli {
    pub fn parsed() -> Cli {
        Cli::parse()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsed_date_arg() {
        let result =
            Cli::try_parse_from(vec!["dailyinfo", "--calendar-date", "2024-01-05"].iter()).unwrap();

        assert_eq!(
            result,
            Cli {
                calendar_date: Some(NaiveDate::from_ymd_opt(2024, 1, 5).unwrap()),
            }
        );
    }
}
