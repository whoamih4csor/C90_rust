/*
AUTHOR: whoamih4csor
DATE: Thursday, March 9, 2023

Exercise 1-4. Write a program to print the corresponding Celsius to Fahrenheit table.

print Celsius-Fahrenheit table
    for cel = 0,20,....,300


FORMULA ->  °F = (°C × 9/5) + 32 

NOTE: this version uses float

to compile:
cargo run --bin exercise_1_4
*/
fn main() {
     // statements
    let (mut fahr, mut celsius): (f32, f32);
    let (lower, upper, step) : (f32, f32, f32);


    //assignment
    lower = 0.0;
    upper = 300.0;
    step  = 20.0;
    celsius = lower;

    //heading
    println!("CELSIUS-FAHRENHEIT");
    // loop, while fahr less than or equal to upper
   while celsius <= upper { 
       fahr = (celsius * (9.0 / 5.0)) + 32.0; // arithmetic operation
       println!("{:3.0}\t{:6.1}", celsius, fahr);
       celsius = celsius + step;
   }
}