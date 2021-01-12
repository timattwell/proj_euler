use rayon::prelude::*;

fn main() {
    println!("{}", check_for_pal());
}

fn check_for_pal() -> i32 {
    let m = (100..1000).into_par_iter()
                         .map(|x| (100..1000).into_par_iter()
                                             .map(|y| y*x)
                                             .filter(|y| is_pal(y))
                                             .max())
                         .max();
    match m {
        Some(max) => match max {
            Some(we) => we,
            None     => panic!("agggggggg")
        },
        None      => panic!("There were no palis"),
    }
}

fn is_pal(n: &i32) -> bool { 
    for 
}
