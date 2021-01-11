/* pr_1 - Tim Attwell
 * 
 * Description: Find the sum of all multiples of three or five
 *      below "n"(in this case 1000).
 * 
 * Notes: Tried parallelising the iter operations using rayon 
 *      but performance gains were very low, even for very large 
 *      values of n
 */

//use rayon::prelude::*;

fn main() {
    let n : i64 = 1000;
    let x = sum_of_3_5_mults(n);
    println!("{}", x);
}

fn sum_of_3_5_mults(n: i64) -> i64 {
    let n = n - 1;
    // Finds sum of all multiples of three
    let threes: i64 = (0..n/3+1).map(|x| x*3)
                                .sum();
    // Finds all multiples of five, bar the ones contained in 
    // the "threes" summation, and sums them
    let fives: i64 = (0..n/5+1).map(|x| x*5)
                               .filter(|z| z % 3 != 0)
                               .sum();
    threes+fives
}
