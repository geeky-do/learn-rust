use std::io;

fn main() -> io::Result<()>{
    let mut weight_on_earth= String::new();
    io::stdin().read_line(&mut weight_on_earth)?;
    let mars_weight : f32 = calculate_weight(100.0);
    println!("Our weight on Mars : {}", mars_weight);
    Ok(())
}


fn calculate_weight(weight: f32) -> f32{

    (weight/9.81)*3.71
}