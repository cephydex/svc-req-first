use std::{fmt, time::SystemTime};

use time::{format_description, OffsetDateTime};

pub const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

pub const URLS:[&str; 9] = [
    "http://megafortunelottery.com",
    "https://api.megafortunelottery.com",
    "https://public-api.megafortunelottery.com/swagger/index.html",
    "https://backend.megafortunelottery.com",
    "https://admin.megafortunelottery.com/",
    "https://backend.mypolicy.market/",
    "https://api.mypolicy.market",
    "https://mypolicy.market",
    "https://admin.mypolicy.market",
];

pub struct RespItem {
    pub site: String,
    pub code: i32
}

impl fmt::Debug for RespItem {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RespItem")
            .field("site", &self.site)
            // .field("code", &self.code)
            .field("code", &format_args!("{}", self.code))
            .finish()
    }
}

pub fn print_date() {
    let dt1: OffsetDateTime = SystemTime::now().into();
    // let dt2 = OffsetDateTime::now_utc();

    let dt_fmt = format_description::parse(DATE_FORMAT_STR).unwrap();

    println!("INIT :: {}", dt1.format(&dt_fmt).unwrap());
    // println!("{}", dt2.format(&dt_fmt).unwrap());
}