#![allow(unused_imports)]
use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::Tz;

fn print_time(tz:Tz, label: &str) {
    let utc_now = Utc::now();
    let local_time: DateTime<Tz> = utc_now.with_timezone(&tz);
    println!("{} {} {}", label, local_time.format("%I:%M %p"), local_time.format("%a %d-%b"));
}

fn main() {
    print_time(chrono_tz::America::New_York, "Maryland");
    print_time(chrono_tz::Asia::Dhaka, "Dhaka");
}
