fn sort_usernames<T: AsRef<str> + Ord>(usernames: &mut Vec<T>) {
    //todo!();

    // solution#1,
    /*usernames.sort_by(|a, b|a.as_ref().to_ascii_lowercase()
        .partial_cmp(&b.as_ref().to_ascii_lowercase())
        .expect("Cannot compare!"));*/

    // or solution#2,
    //usernames.sort_by(|a, b|a.as_ref().to_string().to_ascii_lowercase()
    //    .partial_cmp(&b.as_ref().to_string().to_ascii_lowercase())
    //    .expect("Cannot compare!"));

    // or, instructor's solution
    usernames.sort_by_cached_key(|k|k.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
