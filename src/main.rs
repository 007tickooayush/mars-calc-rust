fn main() {
    println!("Hello, world!");

    println!("{}",calculate_weight_on_mars(74.0));
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}