use std::env;

use r_algorithm::algos::bin_indexed_tree;
use r_algorithm::algos::crane;
use r_algorithm::algos::simple_problem_with_integers;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Fallback to simple_problem_with_integers
    if args.len() < 2 {
        simple_problem_with_integers::run()
    }

    match args[1].as_str() {
        "0" | "crane" => crane::run(),
        "1" | "bin_indexed_tree" => bin_indexed_tree::run(),
        "2" | "simple_problem_with_integers" => simple_problem_with_integers::run(),
        _ => simple_problem_with_integers::run(),
    }
}
