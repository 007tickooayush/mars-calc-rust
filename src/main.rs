fn main() {
    println!("Please enter the weight in KGs:");
    let mut weight = String::new();
    std::io::stdin().read_line(&mut weight).unwrap();
    println!();

    let weight: f32 = weight.trim().parse().expect("A valid number not provided");
    let weight = calculate_weight_on_mars(weight);

    println!("The weight on mars in KGs: {}",weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}