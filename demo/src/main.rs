
use chrono::{DateTime, Duration, Utc, Local};

fn day_earlier(date_time:DateTime<Utc>) ->Option<DateTime<Utc>>{
    date_time.checked_sub_signed(Duration::days(1))
}



fn test_date() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        None => { eprintln!("almost three weeks from now overflows!") }
        Some(x) => { println!("{}", x) }
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("almost three weeks from now overflows!111")
    }
}




fn main() {

    let local_time = Local::now();
    let utc_time =DateTime::<Utc>::from_utc(local_time.naive_utc(),Utc);
}