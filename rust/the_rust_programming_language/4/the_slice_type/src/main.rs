fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    let slice = &s;

    let word = first_word(&s[1..]);

    //s.clear();

    println!("the first word is: {word}");

    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
