fn median(a: Vec<f32>) -> Option<f32> {
    //todo!();
    return match (a.len(), a.len() % 2) {
        // empty list
        (0, _) => None,
        // even list
        (_, 0) => {
            let med = a.len() / 2;
            let mut v = a.clone();
            v.sort_by(|x, y|x.partial_cmp(y).unwrap());
            let num = (v.get(med-1).unwrap_or(&0.0) + v.get(med).unwrap_or(&0.0)) / 2.0;
            Some(num)
        }, 
        // odd list
        (_, 1..) => {
            let med = (a.len() - 1)/2;
            let mut v = a.clone();
            v.sort_by(|x, y|x.partial_cmp(y).unwrap());
            let num = v.get(med).unwrap_or(&0.0);
            Some(*num)
        }
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
