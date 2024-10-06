use chrono::*;

// Assume date specified are in format YYYY-MM-DD and are valid dates
fn weeks_between(a: &str, b: &str) -> i32 {
    //todo!()
    let date1 = match NaiveDate::parse_from_str(a, "%Y-%m-%d") {
        Ok(d1) => d1,
        Err(_) => panic!("ERROR: Cannot parse date string {:?}", a)
    };
    //println!("date1: {:?}", date1);
    let date2 = match NaiveDate::parse_from_str(b, "%Y-%m-%d") {
        Ok(d2) => d2,
        Err(_)=> panic!("ERROR: Cannot parse date string {:?}", b)
    };
    //println!("date2: {:?}", date2);

    let td = date2.signed_duration_since(date1);
    //println!("TD: {:?}", td);
    //let weeks = td.num_weeks() as i32;
    td.num_weeks() as i32
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}