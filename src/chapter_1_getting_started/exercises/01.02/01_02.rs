/*
AUTHOR: whoamih4csor
DATE: Wednesday, March 8, 2023

Exercise 1-2. Experiment to find out what happens when printf's argument
string contains \c,where c is some character not listed above.

NOTE: in this case the printf function doesn't exist in rust
but println! accepts escape sequences 

WARNING: this code this code does not compile

to compile:
cargo run --bin exercise_1_2
*/

fn main(){ //the entry point
    println!("this is a test : \c");// this line causes a error error: unknown character escape: `c`
}