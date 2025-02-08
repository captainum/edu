fn convert32(value: f32) -> f32 {
    (value - 32.0) * 5.0 / 9.0
}

fn convert64(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

fn main() {
    let result32 = convert32(52.0);
    let result64 = convert64(52.0);

    println!("result32: {result32}, result64: {result64}");
}
