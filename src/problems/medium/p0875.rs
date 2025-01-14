// Problem: Koko Eating Bananas
// Tags: Array,  Binary Search

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut left = 1;
    let mut right = *piles.iter().max().unwrap(); // Maximum pile size
    let mut lowest_k = right;

    while left <= right {
        let k = (left + right) / 2;

        // Calculate the total hours needed to eat all piles at speed `k`
        let hours = piles.iter().fold(0, |acc, &pile| acc + (pile + k - 1) / k);

        if hours <= h {
            // If `k` is a valid speed, try to find a smaller one
            lowest_k = k;
            right = k - 1;
        } else {
            // If `k` is too slow, increase the speed
            left = k + 1;
        }
    }

    lowest_k
}
