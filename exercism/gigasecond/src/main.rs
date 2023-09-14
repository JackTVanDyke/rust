// Your task is to determine the date and time one gigasecond (A gigasecond is one thousand million seconds. That is a one with nine zeros after it.) after a certain date.

use time::{PrimitiveDateTime, Duration, Date, Time, Month};

fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {    
    let giga = Duration::seconds(1000000000);

    let result:PrimitiveDateTime = start.checked_add(giga).unwrap();

    result    
}

fn main() {
    let date = Date::from_calendar_date(2015, Month::January, 24).unwrap();
    let time = Time::from_hms(22, 0, 0).unwrap();
    let datetime = PrimitiveDateTime::new(date, time);

    println!("{}", after(datetime))

}
