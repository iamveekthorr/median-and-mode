use std::collections::HashMap;

fn main() {
    let mut my_list = vec![2, 2, 9, 1, 5, 6, 3];

    let res = find_mode_and_median(&mut my_list);
    println!("{:?}", res)
}

fn sort_list(list: &mut Vec<i32>) {
    let len = list.len();

    if len == 0 {
        return;
    }

    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if list[i] > list[i + 1] {
                let prev = list[i];
                list[i] = list[i + 1];
                list[i + 1] = prev;
                sorted = false;
            }
        }
    }
}

fn find_mode_and_median(list: &mut Vec<i32>) -> HashMap<String, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut mode_and_median: HashMap<String, i32> = HashMap::new();

    sort_list(list);

    // find the mode
    for i in 0..list.len() {
        *map.entry(list[i]).or_insert(0) += 1;
    }

    let mut max_frequency = 1;

    for (key, _) in &map {
        let count = map[key];

        if count > max_frequency {
            max_frequency = count;
            mode_and_median.insert(String::from("Mode"), *key);
        }

        if max_frequency == 1 {
            // if all numbers appear exactly once
            mode_and_median.insert(String::from("Mode"), 0); // no mode specified
        }
    }

    // find median
    let middle_index = list.len() / 2;

    if list.len() % 2 == 0 {
        let median = (list[middle_index - 1] + list[middle_index]) / 2;
        mode_and_median.insert(String::from("Median"), median);
    } else {
        list[middle_index];
        mode_and_median.insert(String::from("Median"), list[middle_index]);
    }

    mode_and_median
}
