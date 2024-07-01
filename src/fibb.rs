use std::collections::HashMap;

fn nth_fibb(n: &usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    if [a, b].contains(n) {
        return *n
    } else {
        for _ in 2..=*n {
            (a, b) = (b, a + b);
        }
        b
    }
}

fn fibb_recursive(n: usize) -> usize {
    if n == 0 || n == 1{
        return n;
    } else {
        return fibb_recursive(n - 1) + fibb_recursive(n - 2);
    }
}

enum FibMemo {
    Empty,
    Is(HashMap<u32, u32>),
}

impl Default for FibMemo {
    fn default() -> Self {
        FibMemo::Is(HashMap::from([(0, 0), (1, 1)]))
    }
}


// fn fibb_memoized(n: &usize, memo: &mut FibMemo){
//     todo!
// }

#[cfg(test)]
mod tests{
    use super::*;
    use test_case::test_case;

    #[test_case(0, 0; "Zero")]
    #[test_case(1, 1; "One")]
    #[test_case(2, 1; "Two")]
    #[test_case(3, 2; "Three")]
    #[test_case(10, 55; "Ten")]
    fn test_nth_fibb(n: usize, expected: usize) {
        assert_eq!(nth_fibb(&n), expected);
        assert_eq!(fibb_recursive(n), expected);
    }
}