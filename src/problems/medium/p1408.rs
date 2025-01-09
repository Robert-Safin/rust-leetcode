// Problem: Car Fleet
// Tags: Array, Stack, Sorting, Monotonic Stack

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, f64)> = position
        .iter()
        .zip(speed.iter())
        .map(|(&position, &speed)| (position, (target - position) as f64 / speed as f64))
        .collect();

    cars.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{:?}", cars);

    let mut fleets = 0;
    let mut last_time = 0.0;

    for (_, eta) in cars {
        if eta > last_time {
            last_time = eta;
            fleets += 1;
        }
    }

    fleets
}
