extern crate core;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

/**
A says to S and P: I have chosen two integers x, y such that 1 < x < y and x + y <= 100.
In a moment I will inform S only of s = x + y and P only of p = xy. These announcements will be private.
You are required to determine the pair (x,y). The following conversation takes place.
    1. P says: I do not know the pair
    2. S says I knew you didn't"
    3. P says: Now I know it
    4. S says: I know it too.
Determine (x,y)
 **/

struct Pair {
    x: i32,
    y: i32,
}

fn main() {
    let maxval = 97;

    let mut unique_prods: HashMap<i32, Pair> = HashMap::new();
    let mut dup_products: HashSet<i32> = HashSet::new();

    //requirement for x + y is odd from Goldbach Conjecture.. if  the sum is even then
    // x + y could be the sum of two primes.

    for (x, y) in (2..maxval).tuple_combinations() {
        if x + y > 100 ||
        (x + y) % 2 == 0 ||
        (is_prime(x) && is_prime(y)) ||
        is_prime(x + y - 2) ||
        dup_products.contains(&(x * y))
        {
            continue;
        }
        let product = x * y;

        if unique_prods.contains_key(&product){
            unique_prods.remove(&product);
            dup_products.insert(product);
        }
        else {
            unique_prods.insert(product, Pair{x, y});
        }
    }

    for unique_prod in &unique_prods {
        println!("{}, {}", unique_prod.1.x, unique_prod.1.y);
    }


    let mut sums: Vec<i32> = unique_prods.iter().map(|p| p.1.x + p.1.y).collect();
    sums.sort();
    for s in sums.iter().dedup_with_count(){
        if s.0 == 1{
            let xysum: &i32 = s.1;

            for x in 2 .. (xysum - 2) {
                if unique_prods.contains_key(&(x * (xysum - x))) {
                    println!("({}, {}), sum = {}", x, xysum - x, xysum);
                    break;
                }
            }
        }
    }
}


fn is_prime(val: i32) -> bool{
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        61, 67, 71, 73, 79, 83, 89, 97];

    for p in primes{
        if p > val{
            break;
        }
        if p == val{
            return true;
        }
    }

    false
}

