/*
AUTHOR: whoamih4csor
DATE: Thursday, March 9, 2023
SECTION : 1.2 Variables and arithmetic expressions
print Fahrenheit-Celsius table
    for fahr = 0,20,....,300


FORMULA -> C = (5/9) (F-32)

to compile:
cargo run --bin example_1_2
*/
fn main() {
     // statements
     let (mut fahr, mut celsius): (i32, i32);
     let (lower, upper, step) = (0, 300, 20);
    
     //assignment 
     fahr = lower;

     // loop, while fahr less than or equal to upper
    while fahr <= upper { 
        celsius = 5 * (fahr-32) / 9; // arithmetic operation
        println!("{:}\t{:}", fahr, celsius);
        fahr = fahr + step;
    }

}