// Problem: Maximum Distance in Arrayse
// Tags: Array, Greedy
//
pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let (mut min, mut max) = (
        *arrays.first().unwrap().first().unwrap(),
        *arrays.first().unwrap().last().unwrap(),
    );

    let mut max_distance = 0;

    for arr in arrays.iter().skip(1) {
        let (first, last) = (*arr.first().unwrap(), *arr.last().unwrap());

        max_distance = max_distance.max(last - min);
        max_distance = max_distance.max(max - first);

        min = min.min(first);
        max = max.max(last);
    }

    max_distance
}
