// Problem: Minimize XOR
// Tags: Greedy, Bit Manipulation
pub struct Solution {}
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let n1 = num1.count_ones() as i32;
        let n2 = num2.count_ones() as i32;

        // the strategy here is to take 'num1' as a starting
        // point and flip lowest 1's or 0's depending on
        // which number of set bits (n1 or n2) is larger

        let mut x = num1;

        if n1 != n2 {
            // this solution handles both cases at once
            let flg = (n1 > n2) as i32;
            let mut cnt = (n1 - n2).abs();

            for i in 0..30 {
                if flg == (num1 >> i) & 1 {
                    x ^= 1 << i; // here, the bit is being flipped
                    cnt -= 1;
                }
                if cnt == 0 {
                    break;
                }
            }
        }

        return x;
    }
}
