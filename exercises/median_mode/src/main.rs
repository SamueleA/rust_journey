use std::collections::HashMap;

fn main() {
    let list1 = [1.0, 2.0, 3.0, 1.0, 3.0];
    let list2 = [4.0, 2.0, 3.0, 50.0, 50.0, 50.0];
    let list3 = [6.0, 6.0, 3.0, 4.0, 7.0];

    println!("median of {:?} is {}", list1, get_median(&list1));
    println!("median of {:?} is {}", list2, get_median(&list2));
    println!("median of {:?} is {}", list3, get_median(&list3));

    println!("mode {:?} is {}", list1, get_mode(&list1));
    println!("mode {:?} is {}", list2, get_mode(&list2));
    println!("mode {:?} is {}", list3, get_mode(&list3));
}

fn get_median(list: &[f64]) -> f64 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort_unstable_by(|a, b| a.total_cmp(b));

    if sorted_list.len() % 2 == 0 {
        let midpoint1 = sorted_list.len() / 2 - 1;
        let midpoint2 = sorted_list.len() / 2;
        (sorted_list[midpoint1] + sorted_list[midpoint2]) / 2.0
    } else {
        let midpoint = sorted_list.len() / 2;
        sorted_list[midpoint]
    }
}

// if multiple values have the same occurrence the first once encountered will prevail
fn get_mode(list: &[f64]) -> f64 {
    let mut map = HashMap::<u64, u64>::new();

    for element in list {
        let bits = element.to_bits();
        let count = map.entry(bits).or_insert(0);
        *count += 1;
    }

    let mut mode_value: Option<u64> = None;
    let mut mode: Option<u64> = None;

    for (key, value) in &map {
        match mode_value {
            None => {
                mode_value = Some(*value);
                mode = Some(*key);
            }
            Some(n) => {
                if *value > n {
                    mode_value = Some(*value);
                    mode = Some(*key)
                }
            }
        }
    }

    return f64::from_bits(mode.unwrap_or(0));
}
