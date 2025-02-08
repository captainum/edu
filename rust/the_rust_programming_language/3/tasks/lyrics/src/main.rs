fn main() {
    let days = ["first", "second", "third"];

    let strs = ["A patridge in a pear tree", "Two turtle doves", "Three French hens"];
    let strs_low = ["a patridge in a pear tree", "two turtle doves", "three French hens"];

    let mut counter: usize = 1;

    for day in days {
        println!("On the {} day of Christmas my true love sent to me", day);
        for idx in (0..counter).rev() {
            if idx == 0 {
                if counter != 1 {
                    println!("And {}.", strs_low[idx]);
                }
                else {
                    println!("{}", strs[idx]);
                }
            }
            else {
                println!("{},", strs[idx]);
            }
        }
        println!();
        counter += 1;
    }
}
