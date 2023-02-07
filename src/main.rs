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
    // --gen, --check everyting else will result in help

    if mode == "--gen" {

        let start: u64 = args[2].trim().parse().expect("no valid number on argument 2");
        let end: u64 = args[3].trim().parse().expect("no valid number on argument 3");

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
        println!("
----------------------------------------------------------------------
Prims - Help Page

Desription:
Just a simple tool written in Rust to check / generste prim numbers

Commands:
./prims --help                 |  show the help 
./prims --check <number>       |  check it a number is a prim number 
./prims --gen <start> <end>    |  generate the prim numbers in an area
----------------------------------------------------------------------
");
    }
}

fn is_prim(number: u64) -> bool {
    if number <= 3 {
        return number > 1;
    }


    for i in 2..number {
        if i > (number / 2) {
            // ! thanks to this check there is a ~ 40% speed improvement

            // if it gets in this erea it will be not dividable by any 
            // folowing numbers because x / (x+1) < 2
            return true;
        }
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