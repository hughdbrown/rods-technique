use std::cmp::max;

use knapsack_utils::{
    Item,
    SearchResult,
    // sum_weights,
    sum_values,
    sort_by_density,
};

fn rods_technique_helper(
    items: &[Item],
    i: usize,
    limit_weight: usize,
    weight: usize,
    value: u64,
    //mut max_weight: Box<Result<usize, ()>>,
    mut max_value: u64,
    path: &[usize],
) -> Result<SearchResult, ()> {
    if weight > limit_weight {
        //println!("{weight} > {limit_weight}");
        return Err(());
    }
    if i >= items.len() {
        let r: SearchResult = (path.to_vec(), weight, value);
        //println!("Success: {weight}/{limit_weight} {value}");
        return Ok(r);
    }

    let mut left: Result<SearchResult, ()> = Err(());
    let remaining_items = &items[i..];
    let remaining_value = sum_values(remaining_items);
    if value + remaining_value > max_value {
        // Try with item[i]
        // Make a new copy of the immutable path argument that has `i` appended.
        let mut lpath: Vec<usize> = path.to_vec();
        lpath.push(i);
        let item = &items[i];
        left = rods_technique_helper(
            items,
            i + 1,
            limit_weight,
            weight + item.weight,
            value + item.value,
            //max_weight.clone(),
            max_value,
            &lpath);

        match left {
            Ok(ref t) => { max_value = max(max_value, t.2); }
            Err(_) => {},
        }
    }

    let mut right: Result<SearchResult, ()> = Err(());
    let remaining_items = &items[i+1..];
    let remaining_value = sum_values(remaining_items);
    if value + remaining_value > max_value {
        // Try without item[i]
        right = rods_technique_helper(
            items,
            i + 1,
            limit_weight,
            weight,
            value,
            //max_weight,
            max_value,
            path);
    }

    // Which is better?
    match (left, right) {
        (Ok(lvalue), Ok(rvalue)) => {
            let best_value = if lvalue.2 > rvalue.2 { lvalue } else { rvalue };
            return Ok(best_value);
        },
        (Ok(lvalue), Err(())) => return Ok(lvalue),
        (Err(()), Ok(rvalue)) => return Ok(rvalue),
        (Err(()), Err(())) => return Err(()),
    }
}

fn confirm_blocking(items: &[Item]) {
    let mut bad: Vec<(usize, usize)> = vec![];
    for i in 0..items.len() {
        let item_i = &items[i];
        for j in i+1..items.len() {
            let item_j = &items[j];
            assert!(item_i.density() >= item_j.density());
            if item_i.value == item_j.value {
                // item[i] has to be lower weight for the same value
                if item_i.weight > item_j.weight {
                    bad.push((i, j));
                }
            }
            else if item_i.weight == item_j.weight {
                // item[i] has to be higher value for the same weight
                if item_i.value < item_j.value {
                    bad.push((i, j));
                }
            }
        }
    }
    if bad.len() > 0 {
        println!("Bad {:#?}", bad);
        for (left, right) in bad.iter() {
            println!("({left}, {right}) Left {:?} Right {:?}", &items[*left], &items[*right]);
        }
    }
}

pub fn rods_technique(items: &[Item], limit_weight: usize) -> Result<SearchResult, ()> {
    // Taking some liberties with the original algorithm and its data structures, but
    // this code preserves "blockage" so it should be the same but with less bookkeeping,
    // because:
    //
    //    for all item[i] and item[j] if i < j:
    //    - if item[i].value == item[j].value then item[i].weight <= item[j].weight
    //    - if item[i].weight == item[j].weight then item[i].value >= item[j].value
    //
    // No item that was blocked in the original algorithm can be evaluated before an item that
    // would have blocked it.
    let path = vec![];
    let density_sorted_items: Vec<Item> = sort_by_density(items);
    // confirm_blocking(&density_sorted_items);
    return rods_technique_helper(
        &density_sorted_items,
        0, // i = 0 (start at the first item)
        limit_weight,
        0, // weight = 0 (Weight of current path)
        0, // value = 0 (Value of current path)
        //Box::new(max_weight), // max_weight seen on completed branch
        0, // max_value seen on completed branch
        &path); // Current path (starts empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rods_technique_1() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        match rods_technique(&items, 4) {
            Ok(value) => assert_eq!(value.2, 3500),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn rods_technique_2() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
            Item{value:3300, weight:5},
            Item{value:4000, weight:6},
            Item{value:4200, weight:7},
            Item{value:4400, weight:8},
        ];
        let expected_vals: Vec<u64> = vec![22400, 20900, 20400, 20400, 19400, 19100, 18400, 18200];
        for (i, expected_val) in expected_vals.iter().map(|x: &u64| *x).enumerate() {
            println!("----- Testing {i} for value {expected_val}");
            match rods_technique(&items, 34 - i) {
                Ok(value) => assert_eq!(value.2, expected_val),
                Err(_) => { },
            }
        }
    }
}
