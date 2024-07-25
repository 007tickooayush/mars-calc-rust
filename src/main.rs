fn main() {
    let mut weight = String::new();
    std::io::stdin().read_line(&mut weight).ok();

    let weight: f32 = weight.trim().parse().unwrap();

    println!("{}",calculate_weight_on_mars(weight));
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}