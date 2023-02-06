fn main() {
    let my_number: u64 = 73;
    
    // * values my_number is dividable
    let dividable_by: Vec<u64> = get_dividable_by(my_number);
    println!("{my_number} is dividable by {:?}", dividable_by);
    
    // * check if its a prim
    let prim = is_prim(my_number);
    if prim {
        println!("so its a prim");
    } else {
        println!("its no prim");
    }

    // * gen prims in range
    println!("prims between 2 and 100: ");
    let prims: Vec<u64> = gen_prims(2, 100);
    println!("{:?}", prims);
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