use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`. 
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    //todo!();
    let supported_date_formats: Vec<&str> = vec![
        "%Y-%m-%d",         // 2010-12-11
        "%Y/%b/%d",         // 2010/Dec/11
        "%d.%b.%Y",         // 11.Dec.2010
        "%b.%d.%Y",         // Dec.11.2010
        "%Y $b %d",         // 2010 Dec 11
        "%Y/%B/%d",         // 2010/December/11
    ];
    let mut parsed_date: Option<NaiveDate> = None;
    for date_fmt in supported_date_formats.iter() {
        let dt = NaiveDate::parse_from_str(text.trim(), *date_fmt);
        if dt.is_err() {
            continue;
        } else {
            parsed_date = dt.ok();
            break;
        }
    }
    parsed_date
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(match NaiveDate::from_ymd_opt(2010, 12, 11) {
        Some(d) => d,
        None => panic!("error!")
    }))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(match NaiveDate::from_ymd_opt(1999, 3, 2) {
        Some(d) => d,
        None => panic!("error!")
    }))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(match NaiveDate::from_ymd_opt(2021, 3, 1) {
        Some(d) => d,
        None => panic!("error!")
    }))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(match NaiveDate::from_ymd_opt(2021, 4, 5) {
        Some(d) => d,
        None => panic!("error!")
    }))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}


