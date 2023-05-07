use turing_machine::machine::{Transition, Operation, Machine};
use std::io;

fn main() {
    // L = {a^nb^n : n - 1}
    let program = vec![
        Transition::new(0, 'a', 1, 'x', Operation::Right),
        Transition::new(1, 'a', 1, 'a', Operation::Right),
        Transition::new(1, 'y', 1, 'y', Operation::Right),
        Transition::new(1, 'b', 2, 'y', Operation::Left),
        Transition::new(2, 'y', 2, 'y', Operation::Left),
        Transition::new(2, 'a', 2, 'a', Operation::Left),
        Transition::new(2, 'x', 0, 'x', Operation::Right),
        Transition::new(0, 'y', 3, 'y', Operation::Right),
        Transition::new(3, 'y', 3, 'y', Operation::Right),
        Transition::new(3, ' ', 4, ' ', Operation::Right),
    ];

    let stdin = io::stdin();
    let mut user_input = String::new();

    stdin.read_line(&mut user_input).expect("Err");

    let tape = format!(" {} ", user_input.trim()).chars().collect();
    let mut turing_machine = Machine::new(program, 0, tape, 1, vec![4]);

    match turing_machine.run() {
        true => println!("Accepted"),
        false => println!("Not Accepted"),
    }
}