extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

use std::time::Duration;
use std::thread;


fn main() {
    println!("Hello, guess the number between 1 and 50!");

    let mut count = 0u32;
    let mut suffix = "";
    let secret_number = rand::thread_rng().gen_range(1,10);
    
    loop {
    	count += 1;
    	if count == 1{
    		suffix = " and you are amazing!"
    	} else {suffix = "es!"}
	    println!("Please input your guess.");

	    let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");

	    let guess: u32 = match guess.trim().parse(){
	    	Ok(num) => num,
	    	Err(__)	=> continue,
	    };

	   
	    match guess.cmp(&secret_number) {
	    	Ordering::Less		=> println!("Too small!"),
	    	Ordering::Greater	=> println!("Too big!"),
	    	Ordering::Equal		=> {
	    		println!("You win and it took you {} guess{}", count, suffix);
	    		println!("Thank you for playing.. closing in 10 seconds.");
	    		use std::thread;
	    		thread::sleep(Duration::from_millis(10000));
	    		break;
	    	}
	    }
    }
}