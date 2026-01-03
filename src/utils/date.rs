use std::time::SystemTime;
use time::OffsetDateTime;

pub fn format_date(t: SystemTime) -> String {
    let fmt =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    OffsetDateTime::from(t).format(&fmt).unwrap()
}
