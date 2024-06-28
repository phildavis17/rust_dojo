// use core::num;
use std::collections::{HashMap, HashSet};

pub fn primes_main() {
    println!("{}", is_prime(&11));
    println!("{:?}", primes_up_to(&11));
    println!("{}", 11 % 7);
    println!("{:?}", over_or_under_six(&10000));
}


fn primes_up_to(n: &usize) -> Vec<usize> {
    let mut nums = HashSet::<usize>::from_iter(2..=*n);
    let upper_bound = *n as f64;
    for factor in 2..=upper_bound.sqrt() as usize {
        if !nums.contains(&factor) {
            continue;
        }
        let mut f = factor;
        while &f < n {
            f += &factor;
            nums.remove(&f);
        }
    }
    let mut ret = Vec::from_iter(nums);
    ret.sort_unstable();
    return ret
}

fn is_prime(n: &usize) -> bool {
    let max_factor = (*n as f64).sqrt() as usize;
    let prime_factors = primes_up_to(&max_factor);
    for f in prime_factors {
        if n % f == 0 {
            return false;
        }
    }
    return true
}

fn over_or_under_six(limit: &usize) -> HashMap<usize, usize> {
    let mut count = HashMap::from([(1, 0), (5, 0)]);
    for p in &primes_up_to(&limit)[2..] {
        if let Some(c) = count.get_mut(&(p % 6)) {
            *c += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, vec![2]; "Two")]
    #[test_case(4, vec![2, 3]; "Composite Number")]
    #[test_case(11, vec![2, 3, 5, 7, 11]; "Prime Number")]
    #[test_case(16, vec![2, 3, 5, 7, 11, 13]; "Perfect Square")]
    fn test_primes_up_to(n: usize, expected: Vec<usize>) {
        assert_eq!(primes_up_to(&n), expected);
    }

    #[test_case(2, true; "Two")]
    #[test_case(6, false; "Composite Number")]
    #[test_case(16, false; "Perfect Square")]
    #[test_case(49, false; "Perfect Square of a prime")]
    #[test_case(1000000009, true; "Big Prime")]
    #[test_case(100160063, false; "Big Semiprime")]
    fn test_is_prime(n: usize, expected: bool) {
        assert_eq!(is_prime(&n), expected)
    }
}