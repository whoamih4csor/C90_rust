/*
AUTHOR: whoamih4csor
DATE: Thursday, March 9, 2023

Exercise 1-3. Modify the temperature conversion program to print a heading
above the table.

print Fahrenheit-Celsius table
    for fahr = 0,20,....,300


FORMULA -> C = (5/9) (F-32)

NOTE: this program uses float:f32

to compile:
cargo run --bin exercise_1_3
*/
fn main() {
    // statements
    let (mut fahr, mut celsius): (f32, f32);
    let (lower, upper, step) : (f32, f32, f32);


    //assignment
    lower = 0.0;
    upper = 300.0;
    step  = 20.0;
    fahr = lower;

    //heading
    println!("FAHRENHEIT-CELSIUS");
    // loop, while fahr less than or equal to upper
   while fahr <= upper { 
       celsius = (5.0 / 9.0) * (fahr - 32.0); // arithmetic operation
       println!("{:3.0}\t{:6.1}", fahr, celsius);
       fahr = fahr + step;
   }

}