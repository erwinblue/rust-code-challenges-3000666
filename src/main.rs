/*
    EXERCISE:
    1. Implement the ImportantEvent struct
       Must have event's name and a date
    2. Implement the Deadline trait for ImportantEvent struct

    Notes:
    * Use chrono crate
 */

// TODO: import the necessary dependencies
use chrono::prelude::*;
use chrono::Duration;


struct ImportantEvent {
    // TODO: define data structure
    what: String,
    when: DateTime<Local>
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    // TODO: implement trait
    fn is_passed(&self) -> bool {
        //if self.when.signed_duration_since(Local::now()) < Duration::zero() {
        if self.when.signed_duration_since(Local::now()) < Duration::zero() {
            true
        } else {
            false
        }
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        // deprecated Local.ymd function
        //when: Local.ymd(2020, 12, 25)
        when: Local.with_ymd_and_hms(2020, 12, 25, 0, 0, 0)
            .single()
            .unwrap_or(Local::now())
    };
    
    if missed_christmas.is_passed() {
        // Make the message better...
        //println!("oh well, maybe next year");
        println!("Oh well. \'{}\' on \'{}\' has passed.  Maybe next year.", 
            missed_christmas.what, 
            missed_christmas.when.format("%B %d, %Y")
        )
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        // fix deprecated functions
        //when: Local::today() - Duration::hours(25),
        when: Local::now() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;
    
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        // fix deprecated functions
        //when: Local::today() + Duration::hours(25),
        when: Local::now() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}
