use std::collections::HashMap;


fn main() {
    println!("========================================");
    println!("Welcome to another fun implementation being done in rust");
    println!("Mode, Median and Mean calculator for a unidimensional dataset");
    println!("=========================================");

    println!("Here we go again...");

    let ages = vec![20, 20, 21, 20, 27, 23, 22, 21, 21];
    println!("Input data {:#?}", ages);
    
    println!("\nResults: ");
    println!("---------------------------------------");
    match mode(&ages) {
        Some(m) => println!("Mode for the dataset is: {:?}", m),
        None => println!("Mode cannot be found for an empty dataset"),
    }
    println!("Median for the dataset is: {:#?}", median(ages.clone()));
    println!("Mean for the dataset is: {:#?}", mean(&ages));
}

fn mode(arr: &Vec<i64>) -> Option<i64> {
    if arr.is_empty() {
        return None;
    }

    let mut weights = HashMap::new();

    for &val in arr {
        *weights.entry(val).or_insert(0) += 1;
    }
    
    weights
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)

}

fn mean(arr: &Vec<i64>) -> f64 {
    let mut result = 0;
    let len = arr.len() as f64;

    for val in arr {
        result += val;
    }

    return result as f64 / len; 
}

fn median(mut arr: Vec<i64>) -> i64 {
    arr.sort();
    // println!("Sorted arr: {:?}", arr);

    let len = arr.len() as i64;
    let mid = len / 2;
    
    let res = if len % 2 != 0 { 
        &arr[mid as usize] 
    } else { 
        &((&arr[mid as usize] + &arr[((len + 1) / 2) as usize]) / 2)
    };
    
    return *res;
}
