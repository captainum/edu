fn main() {
    {
        let mut s = String::new();
    }

    {
        let data = "initial contents";

        let s = data.to_string();

        let s = "initial contents".to_string();
    }

    {
        let s = String::from("initial contents");
    }

    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שלום");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
    }

    {
        let mut s = String::from("lo");
        s.push('l');
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
    }

    {
        let s1 = String::from("hello");
        // error
        // let h = s1[0];
    }

    {
        // String is Vec<u8>
        let hello = String::from("Hola");  // 4 bytes long
        let hello = String::from("Здравствуйте");  // 24 bytes long
    }

    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];
        println!("{s}");

        //  panic
        // let s = &hello[0..3];
    }

    {
        for c in "Здv".chars() {
            println!("{c}");
        }

        for b in "Здv".bytes() {
            println!("{b}");
        }
    }
}
