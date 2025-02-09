// !!!REFERENCES MUST ALWAYS BE VALID!!!

fn main() {
    {
        let s1 = String::from("hello");    

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    {
        //let s = String::from("hello");
        let mut s = String::from("hello");
        change(&mut s);
    }

    //{
    //    let mut s = String::from("hello");

    //    let r1 = &mut s;
    //    let r2 = &mut s;

    //    println!("{}, {}", r1, r2);
    //}

    //{
    //    let mut s = String::from("hello");

    //    let r1 = &s;
    //    let r2 = &s;
    //    let r3 = &mut s;

    //    println!("{}, {} and {}", r1, r2, r3);
    //}

    {
        let mut s = String::from("hello");

        // Note that a reference’s scope starts from where it is introduced
        // and continues through the last time that reference is used
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        println!("{r1} and {r2}");
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{r3}");
        // error
        //println!("{r1}, {r2} and {r3}");
    }

    {
        // error
        // let reference_to_nothing = dangle();
    }

    {
        let no_reference = no_dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// error here
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
