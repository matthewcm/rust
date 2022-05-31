use std::io;



fn main() {

    println!("Enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Input: {}", input);

    let weight: f32 = input.trim().parse().unwrap();
    println!("{}", weight);

    let mut mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Our weight on Mars! {}kg", mars_weight);

    mars_weight = mars_weight * 100.0;
    println!("Our new weight! {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}


