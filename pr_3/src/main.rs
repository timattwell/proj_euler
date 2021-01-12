/* pr_3 - Tim Attwell
 *
 * Description - The prime factors of 13195 are 5, 7, 13 and 29.
 *          What is the largest prime factor of the number 600851475143
 * 
 * Notes - Two functions this time to make things slightly easier to follow.
 *      The first finds if a number is prime
 */
use num_integer::Roots;
use rayon::prelude::*;

fn main() {
    let n: i64 = 600851475143;
    println!("{}", factors(n));
}

fn factors(n: i64) -> i64 {
    let p_factors = (2..n/2+1).into_par_iter()
                              .filter(|x| x % 2 != 0)
                              .filter(|x| n % x == 0)
                              .filter(|y| is_prime(y))
                              .max();
                                 
    match p_factors {
        Some(max) => max,
        None      => panic!("This number is already a prime!"),
    }
    
}

fn is_prime(n: &i64) -> bool {

    let factors = (2..n.sqrt()+1).into_par_iter()
                                 .filter(|x| x % 2 != 0)
                                 .filter(|x| n % x == 0)
                                 .min();
    match factors {
        Some(_) => false,
        None   => true,
    }

}
