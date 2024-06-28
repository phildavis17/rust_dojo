use std::collections::HashMap;

fn nth_fibb(n: &usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    if [a, b].contains(&n) {
        return *n
    } else {
        for _ in 2..=*n {
            (a, b) = (b, a + b);
        }
        b
    }
}

fn fibb_recursive(n: &usize) -> Vec<usize> {
    todo!()
}

// pub struct FibMemo{
//     memo: HashMap<usize, usize>
// }

type FibMemo = HashMap<usize, usize>;

impl Default for FibMemo {
    fn default() -> Self {
        Self{
            memo: HashMap::from([(0, 0), (1, 1)]),
        }
    }
}

fn fibb_memoized(n: &usize, memo: FibMemo){
    todo!()
}

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
        assert_eq!(nth_fibb(&n), expected)
    }
}