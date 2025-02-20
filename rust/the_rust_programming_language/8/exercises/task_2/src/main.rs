// eng only

fn convert(s: &mut String) {
    if s.len() == 0 {
        return;
    }

    let vowels: Vec<&str> = vec!["a", "e", "i", "o", "u", "y"];
    let consonants: Vec<&str> = vec!["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z"];

    let first_char = &s[0..1].chars().as_str();
    let mut arr: Option<&Vec<&str>> = None;
    let mut idx = 0;
    for i in &vowels {
        if first_char == i {
            arr = Some(&vowels);
            break;
        }
    }

    match arr {
        None => {
            for i in &consonants {
                if first_char == i {
                    arr = Some(&consonants);
                    break;
                }
                idx += 1;
            }
        },
        _ => {},
    }

    match arr {
        Some(res) => {
            if res.len() == 6 {
                s.push_str("-hay");
            }
            else {
                s.remove(0);
                let tmp = format!("-{}{}", res[idx], "ay");
                s.push_str(&tmp);
            }
        },
        _ => {},
    }

}

fn main() {
    let mut word = String::from("");
    convert(&mut word);
    assert_eq!(word, "");

    let mut word = String::from("first");
    convert(&mut word);
    assert_eq!(word, "irst-fay");

    let mut word = String::from("apple");
    convert(&mut word);
    assert_eq!(word, "apple-hay");
}
