use std::io;

fn main() -> io::Result<()>{
    println!("Enter weight: ");
    let mut weight_input= String::new();

    io::stdin().read_line(&mut weight_input)?;
    let weight_float : f32 = weight_input.trim().parse()?;
    println!("Your input was: {}", weight_float);
    let mars_weight : f32 = calculate_weight(weight_float);
    println!("Our weight on Mars : {}", mars_weight);
    Ok(())
}


fn calculate_weight(weight: f32) -> f32{

    (weight/9.81)*3.71
}