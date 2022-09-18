use std::env;
use learn_command_line::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let question = &args[1..args.len()].join(" ");
    let answer = run(question);
    println!("{}", answer);
}
