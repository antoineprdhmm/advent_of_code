use crate::read_input;

pub fn read_times() -> (u128, Vec<u128>) {
    let lines: Vec<String> = read_input(2020, 13)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    let arrival_time = lines[0].parse::<u128>().unwrap();
    let bus_loop_times = lines[1]
        .split(',')
        .filter(|v| *v != "x")
        .map(|v| v.parse::<u128>().unwrap())
        .collect();

    (arrival_time, bus_loop_times)
}

pub fn read_bus_loop_times_with_index() -> Vec<(u128, u128)> {
    let lines: Vec<String> = read_input(2020, 13)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, v)| {
            if v == "x" {
                return None;
            }
            Some((v.parse::<u128>().unwrap(), i as u128))
        })
        .collect()
}

pub fn run_part_1() {
    let (arrival_time, bus_loop_times) = read_times();

    let (bus_id, waiting_time) = bus_loop_times
        .iter()
        .map(|bus_loop_time| {
            let nb_complete_loops_before_arrival = arrival_time / bus_loop_time;
            let nb_complete_loops = if arrival_time % bus_loop_time == 0 {
                // bus arrived at same time, no waiting time
                nb_complete_loops_before_arrival
            } else {
                // previous loop ended before arrival, need to wait for the bus
                nb_complete_loops_before_arrival + 1
            };
            let waiting_time = nb_complete_loops * bus_loop_time - arrival_time;
            (bus_loop_time, waiting_time)
        })
        .min_by_key(|(_, waiting_time)| *waiting_time)
        .expect("Bus not found");

    assert_eq!(bus_id * waiting_time, 1835);
}

pub fn run_part_2() {
    let bus_loop_times_with_index = read_bus_loop_times_with_index();

    let big_m: u128 = bus_loop_times_with_index
        .iter()
        .map(|(time, _)| time)
        .product();

    /*
        Chinese Reminder Thorem:
        For input like `17,x,13,19`
        We are looking to solve this system
        x       = 0 mod 17
        x + 2   = 0 mod 13
        x + 3   = 0 mod 19
    */

    // doing `% big_m` at various places, and using `fold` instead of `sum` to avoid overflow
    let x = bus_loop_times_with_index
        .iter()
        .map(|(time, index)| {
            let m = big_m / time;
            let a = (time - index % time) % time;
            let modular_inverse = modinverse(m as i128, *time as i128).unwrap() as u128;
            (((a * m) % big_m) * modular_inverse) % big_m
        })
        .fold(0, |acc, x| (acc + x) % big_m);

    assert_eq!(x, 247086664214628);
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x, y) = egcd(b, a % b);
    (g, y, x - (a / b) * y)
}

fn modinverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        return None;
    }
    Some((x % m + m) % m)
}
