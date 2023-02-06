//use std::io;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let rescue_mode: String = String::from("--help");

    let mut mode: &String = &rescue_mode;

    if args.len() >= 2 {
        mode = &args[1];

        if mode == "--gen" && args.len() < 4 {
            println!("not enought arguments!");
            mode = &rescue_mode;
        } else if mode == "--check" && args.len() < 3 {
            println!("not enought arguments!");
            mode = &rescue_mode;
        }
    }
    
        

    // valid modes:
    // --gen, --check, --help

    if mode == "--gen" {

        let start: u64 = args[2].trim().parse().expect("no valid number on argument 2");
        let end: u64 = args[3].trim().parse().expect("no valid number on argument 3");

        /*
        let mut start = String::new();
        let mut end = String::new();

        println!("Where should the Prim Generator Start?");
        io::stdin().read_line(&mut start)
            .expect("couldnt read line");
        
        let start: u64 = start.trim().parse().expect("no valid number");

        println!("Where should the Prim Generator Stop?");
        io::stdin().read_line(&mut end)
            .expect("couldnt read line");

        let end: u64 = end.trim().parse().expect("no valid number");
        
         */

        println!("Prims between {start} and {end}: ");
        let prims: Vec<u64> = gen_prims(start, end);
        println!("{:?}", prims);
    } else if mode == "--check" {
        let my_number: u64 = args[2].trim().parse().expect("no valid number on argument 2");
    
        let prim = is_prim(my_number);
        if prim {
            println!("{my_number} is a prim number");
        } else {
            println!("{my_number} is no prim number!");

            let dividable_by: Vec<u64> = get_dividable_by(my_number);
            println!("Because {my_number} is dividable by {:?}", dividable_by);
            println!("But a prime number needs to be only dividable by 1 and itself!");
        }
        
    } else {
        println!("-----------------------------------------------------------------");
        println!("Help Page");
        println!("Desription: ");
        println!("Just a simple tool written in Rust to check / generste prim numbers");
        println!("Commands: ");
        println!("./prims --help                 |  show the help");
        println!("./prims --check <number>       |  check it a number is a prim number");
        println!("./prims --gen <start> <end>    |  generate the prim numbers in an area");
        println!("-----------------------------------------------------------------");
    }
}

fn is_prim(number: u64) -> bool {
    for i in 2..number {
        if number % i == 0 {
            // returns false if its dividable by i
            // 1 & number are exclusive
            return false;
        }
    }
    return true;
}

fn get_dividable_by(number: u64) -> Vec<u64> {
    let mut dividable_by: Vec<u64> = vec![];
    for i in 1..(number + 1) {
        if number % i == 0 {
            // dividable by i |  1 & number are exclusive here
            dividable_by.insert(0, i);
        }
    }
    dividable_by
}

fn gen_prims(start: u64, end: u64) -> Vec<u64> {
    // inclusive start and end
    let mut prim_vec: Vec<u64> = vec![];
    for i in start..(end + 1) {
        if is_prim(i) {
            prim_vec.insert(0, i);
        }
    }
    prim_vec
}