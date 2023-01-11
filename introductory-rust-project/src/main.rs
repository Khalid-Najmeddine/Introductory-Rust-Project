#![deny(clippy::all)]

fn main() {
    let first_name = "John";
    println!("Your name is {}", first_name);

    let age = 30;
    println!("Your age is {}", age);

    let population = 62000000;
    println!("The population of the city is {}", population);

    let red = 0xFA;
    println!("Red is {:x}", red);
    let rgb = 0xFF0000;
    println!("RGB is {:x}", rgb);

    let distance_kilometers = 5.5;
    println!("The distance is {} kilometers", distance_kilometers);

}
