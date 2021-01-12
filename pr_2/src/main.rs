/* pr_2 - Tim Attwell
 * 
 * Description - Find the sum of all the even fibonacci numbers that
 *      do not exceed 4 million.
 * 
 * Notes - Not the cleanest solution. May come back and do something 
 *      with match statements and recurrcive functions. But lets be
 *      fair, I've calculated the fibonacci sequence at least 50 times
 *      in a dozen different ways over my life...
 * 
 */ 


fn main() {
    let lim: i64 = 4e6 as i64;
    println!("{}", even_fibo_sum(lim));
}

fn even_fibo_sum(lim: i64) -> i64 {
    let mut val1 = 1;
    let mut val2 = 1;
    let mut tmp;
    let mut sum = 0;
    loop {
        tmp = val2;
        val2 = val1 + val2;
        val1 = tmp;
        if val2 >= lim {
            break
        }
        if val2 as f64 % 2. == 0. {
            sum += val2;
        }
    }
    sum
}
