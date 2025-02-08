use std::io;

fn main() {
    {
        let sum = 5 + 10;

        let difference = 95.5 - 4.3;

        let product = 4 * 30;

        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;

        let remainder = 43 % 5;

        println!("Sum: {sum}\nDifference: {difference}\nProduct: {product}\nQuotient: {quotient}\nTruncated: {truncated}\nRemainder: {remainder}");
    }

    println!("");

    {
        // !!!CHAR IS 4 BYTES LONG!!!
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("C: {c}\nZ: {z}\nHeart eyed cat: {heart_eyed_cat}");
    }

    println!("");

    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;

        println!("X: {x}, Y: {y}, Z: {z}");

        // No way to use like this:
        // println!("five_hundred: {tup.0}, six_point_four: {tup.1}, one: {tup.2}");

        println!("five_hundred: {}, six_point_four: {}, one: {}", tup.0, tup.1, tup.2);

        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
    }

    println!("");

    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is {element}");

        let b = [3; 5]; // [3, 3, 3, 3, 3]

        let months = [
            "January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"
        ];
    }
}
