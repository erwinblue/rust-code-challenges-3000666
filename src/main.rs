
use std::cmp::PartialEq;

/*fn unique(a: Vec<i32>) -> Vec<i32> {
    //todo!();

    /* 
        Solution #1: Own Implementation of unique 
    */

    /*let mut ret_vec: Vec<i32> = Vec::new();
    match a.is_empty() {
       true => return a,
       false => ()
    };
    for (x, y) in a.iter().enumerate() {
        //println!("-----\n\nElem {:?} value {:?}", x, *y);
        match x {
            0 => {ret_vec.push(*y); continue;},
            1.. => {}
        };
        let mut is_unique = true;
        for z in ret_vec.iter() {
            if *y == *z {
                is_unique = false;
            }
        } 
        if is_unique {
            ret_vec.push(*y);
        }
    }
    ret_vec.sort();
    return ret_vec;*/

    /* 
        Solution #2: Use Rust modules for simple Vec<i32> 
    */
    let mut v = a.clone();
    v.sort();
    v.dedup();
    v
}
*/

/*
    Advanced 1: use generic types solution
 */
fn unique<T: Copy + Ord + PartialEq>(a: Vec<T>) -> Vec<T> {
    //todo!();
    /* 
        Solution #1: Own Implementation of unique 
    */
    /*let mut ret_vec: Vec<T> = Vec::new();
    match a.is_empty() {
       true => return a,
       false => ()
    };
    for (x, y) in a.iter().enumerate() {
        //println!("-----\n\nElem {:?} value {:?}", x, *y);
        match x {
            0 => {ret_vec.push(*y); continue;},
            1.. => {}
        };
        let mut is_unique = true;
        for z in ret_vec.iter() {
            if *y == *z {
                is_unique = false;
            }
        } 
        if is_unique {
            ret_vec.push(*y);
        }
    }
    ret_vec.sort();
    return ret_vec;*/

    /* 
        Solution #2: Use Rust modules for simple Vec<i32> 
    */
    let mut v = a.clone();
    v.sort();
    v.dedup();
    v

}

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}


#[test]
fn empty_list() {
    //let input = vec![];
    //let expected_output = vec![];
    let input: Vec<i32>= vec![];
    let expected_output = Vec::new();
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
