struct User {
    username: String,
    age: u8
}

fn main() {
    {
        let config_max: Option<u8> = None;
        //let config_max = Some(3u8);

        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => (),
        }

        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
        else {
            println!("It is None!");
        }
    }

    {
        let x = 6;
        //let x = 7;

        if let 7 = x {
            println!("It is 7!");
        }
        else {
            println!("It is not 7:(");
        }
    }
}
