//use rayon::prelude::*;

fn main() {

    let n : usize = 1000;

    let x = mult_of_3_5(n);

    println!("{}", x);



}

fn mult_of_3_5(n: usize) -> i64 {
    let n: i64 = (n - 1) as i64;
    let threes: i64 = (0..n/3+1).map(|x| x*3)
                              .sum();
    let fives: i64 = (0..n/5+1).map(|x| x*5)
                             .filter(|z| z % 3 != 0)
                             .sum();
    threes+fives
}
