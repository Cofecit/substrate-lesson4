#![allow(dead_code, unused_imports)]

use std::ops::ControlFlow;

pub fn sum_u32_numbers(numbers: &[u32]) -> Option<u32> {
    match numbers
        .iter() // as iterator
        .try_fold(0_u32, |prev, num| {
            if let Some(next) = prev.checked_add(*num) {
                ControlFlow::Continue(next)
            } else {
                ControlFlow::Break(prev)
            }
        }) {
        ControlFlow::Continue(sum) => Some(sum),
        ControlFlow::Break(_) => None,
    }
}

/////////////////////////// test case for sum of u32 numbers ///////////////////////////
#[cfg(test)]
mod tests {
    use crate::u32_sum::sum_u32_numbers;

    #[test]
    fn test_u32_sum() {
        // sum of 1 util 5 is 15
        let numbers: [u32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32_numbers(&numbers), Some(15));

        // sum of 1 util 4 and max u32 will overflow
        let numbers: [u32; 5] = [1, 2, 3, 4, u32::MAX];
        assert_eq!(sum_u32_numbers(&numbers), None);
    }
}
