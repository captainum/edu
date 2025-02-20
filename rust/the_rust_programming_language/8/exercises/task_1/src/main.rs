use std::collections::HashMap;

fn median(v: &Vec<i32>) -> Option<&i32> {
    if v.len() == 0 {
        return None;
    }

    let len = v.len();
    let mut v1 = v.to_vec();

    v1.sort();

    let median = &v1[len / 2];

    let mut i = 0;
    loop {
        if v[i] == *median {
            break Some(&v[i]);
        }
        i += 1;
    }
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut res = match v.len() {
        0 => return None,
        _ => 0,
    };
    let mut max_count = -1;

    let mut counts = HashMap::new();
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
        }
    }

    let mut cnt = 0;
    for (k, v) in &counts {
        if *v == max_count {
            cnt += 1;
            res += *k;
        }
    }

    Some(res / cnt)
}

fn main() {
    let v = Vec::new();
    match median(&v) {
        None => assert_eq!(1, 1),
        _ => assert_eq!(0, 1),
    }
    match mode(&v) {
        None => assert_eq!(1, 1),
        _ => assert_eq!(0, 1),
    }

    let v = vec![2, 1, 4, 5, 3];
    match median(&v) {
        Some(res) => assert_eq!(*res, 3),
        None => assert_eq!(0, 1),
    }
    match mode(&v) {
        Some(res) => assert_eq!(res, 3),
        None => assert_eq!(0, 1),
    }

    let v = vec![3, 2, 1, 4];
    match median(&v){
        Some(res) => assert_eq!(*res, 3),
        None => assert_eq!(0, 1),
    }
    match mode(&v) {
        Some(res) => assert_eq!(res, 2),
        None => assert_eq!(0, 1),
    }

    let v = vec![1, 2, 3, 4, 2];
    match median(&v){
        Some(res) => assert_eq!(*res, 2),
        None => assert_eq!(0, 1),
    }
    match mode(&v) {
        Some(res) => assert_eq!(res, 2),
        None => assert_eq!(0, 1),
    }

    let v = vec![1, 2, 3, 4, 2, 4];
    match median(&v){
        Some(res) => assert_eq!(*res, 3),
        None => assert_eq!(0, 1),
    }
    match mode(&v) {
        Some(res) => assert_eq!(res, 3),
        None => assert_eq!(0, 1),
    }
}
