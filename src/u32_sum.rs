#![allow(dead_code, unused_imports)]
pub fn sum_u32_numbers(numbers: &[u32]) -> Option<u32> {
    let sum = numbers
        .iter() // as iterator
        .fold((0, false), |left: (u32, bool), right: &u32| { // use fold to sum numbers, return (sum, is_overflowing)
            left.0.overflowing_add(*right)
        });
    
    // when overflowing, return None, otherwise return Some(sum)    
    if sum.1 {
        return None;
    } else {
        return Some(sum.0);
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
