#[path = "algorithms/recursive.rs"] pub mod recursive;
#[path = "algorithms/searching.rs"] pub mod searching;
#[path = "algorithms/sorting.rs"] pub mod sorting;

pub mod utils;

use recursive::*;
use searching::*;
use sorting::*;
use utils::*;

fn main() {
	println!("Welcome! Learn different algorithms with this CLI.\n");
    loop {
		println!("{}\n{}\n{}\n{}",
				"0 - Exit",
				"1 - Recursive",
				"2 - Searching",
				"3 - Sorting");
		let alg_grp = input(&"\nWhich algorithm group you wanna use? ");
		let alg_grp: u32 = alg_grp.trim().parse()
						.expect("Please enter a number!");
		
		match alg_grp {
			0 => {
				println!("Bye!");
				break;
			},
			1 => {
				loop {
					println!("\n{}\n{}\n{}",
							"0 - Go back",
							"1 - Get factorial of a number",
							"2 - Get Fibonacci term");
					let algonum = input(&"\nWhat do u wanna do now? ");
					let algonum: u32 = algonum.trim().parse()
									.expect("Please enter a number!");
					
					match algonum {
						0 => break,
						1 => {
							let number = input(&"\nEnter a number : ");
							let number: u32 = number.trim().parse()
											.expect("Please enter a number!");

							println!("Factorial of {} is {}", number,
									factorial::get(number));
						},
						2 => {
							let number = input(&"\nEnter a number : ");
							let number: u32 = number.trim().parse()
											.expect("Please enter a number!");

							println!("{}th term is {}", number,
									fibonacci::get(number));
						},
						_ => {
							println!("\nInvalid");
							continue;
						}
					};
				}
			},
			2 => {
				loop {
					println!("\n{}\n{}\n{}\n{}",
							"0 - Go back",
							"1 - Linear Search",
							"2 - Binary Search",
							"3 - Jump Search");
					let algonum = input(&"\nWhich searching algorithm to use? ");
					let algonum: u32 = algonum.trim().parse()
									.expect("Please enter a number!");

					let mut array: [u32; 10] = [0; 10];
					randomizer::rand_arr(&mut array);

					let search = randomizer::gen_rand_num();

					let loc: i32 = match algonum {
						0 => break,
						1 => linear::search(&array, search),
						2 => binary::search(&mut array, search),
						3 => jump::search(&mut array, search),
						_ => {
							println!("\nInvalid");
							continue;
						}
					};

					println!("\n{:?}", array);

					if loc != -1 {
						println!("{} is present at {}", search, loc);
					} else {
						println!("{} is not found", search);
					}
				}
			},
			3 => {
				loop {
					println!("\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
							"0 - Go back",
							"1 - Bubble Sort",
							"2 - Insertion Sort",
							"3 - Selection Sort",
							"4 - Merge Sort",
							"5 - Shell Sort",
							"6 - Quick Sort");
					let algonum = input(&"\nWhich sorting algorithm to use? ");
					let algonum: u32 = algonum.trim().parse()
								.expect("Please enter a number!");
					
					let mut array: [u32; 10] = [0; 10];
					randomizer::rand_arr(&mut array);

					println!("\n{:?}", array);

					match algonum {
						0 => break,
						1 => bubble::sort(&mut array),
						2 => insertion::sort(&mut array),
						3 => selection::sort(&mut array),
						4 => merge::sort(&mut array),
						5 => shell::sort(&mut array),
						6 => quick::sort(&mut array),
						_ => {
							println!("\nInvalid");
							continue;
						}
					};

					println!("{:?}", array);
				}
			},
			_ => {
				println!("\nInvalid");
				continue;
			}
		};
		println!();
	}
}
