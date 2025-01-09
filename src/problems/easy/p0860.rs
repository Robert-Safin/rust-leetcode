// Problem: Lemonade Change
// Tags: Array, Greedy

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut count_5 = 0;
    let mut count_10 = 0;

    for bill in bills {
        match bill {
            5 => count_5 += 1,
            10 => {
                if count_5 > 0 {
                    count_5 -= 1;
                    count_10 += 1;
                } else {
                    return false;
                }
            }
            20 => {
                if count_10 > 0 && count_5 > 0 {
                    count_10 -= 1;
                    count_5 -= 1;
                } else if count_5 >= 3 {
                    count_5 -= 3;
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    true
}
