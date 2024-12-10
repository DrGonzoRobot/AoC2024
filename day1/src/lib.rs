use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn solve1(inp: &str) -> PyResult<String> {
    let mut list1 = vec![];
    let mut list2 = vec![];

    let lines: Vec<&str> = inp.split('\n').collect();

    for line in lines {
        let cols: Vec<&str> = line.split("   ").collect();
        if cols.len() != 2 {
            continue;
        }
        match (cols[0].trim().parse::<i32>(), cols[1].trim().parse::<i32>()) {
            (Ok(first_col), Ok(second_col)) => {
                list1.push(first_col);
                list2.push(second_col);
            }
            _ => {
                eprintln!("ERROR")
            }
        }
    }

    list1.sort();
    list2.sort();

    let mut diffs = vec![];

    for i in 0..list1.len() {
        diffs.push((list1[i]-list2[i]).abs());
    }

    let result: i32 = diffs.iter().sum();

    Ok(format!("{}", result))
}

struct CountingHashMap<T: Eq + std::hash::Hash> {
    map: HashMap<T, usize>
}

impl<T: Eq + std::hash::Hash> CountingHashMap<T> {
    fn new() -> Self {
        CountingHashMap {
            map: HashMap::new()
        }
    }

    fn increment(&mut self, item: T) {
        *self.map.entry(item).or_insert(0) += 1;
    }

    fn get_count(&self, item: &T) -> usize {
        *self.map.get(item).unwrap_or(&0)
    }
}

#[pyfunction]
fn solve2(inp: &str) -> PyResult<String> {

    let mut list1 = vec![];
    let mut list2 = vec![];

    let lines: Vec<&str> = inp.split('\n').collect();

    for line in lines {
        let cols: Vec<&str> = line.split("   ").collect();
        if cols.len() != 2 {
            continue;
        }
        match (cols[0].trim().parse::<i32>(), cols[1].trim().parse::<i32>()) {
            (Ok(first_col), Ok(second_col)) => {
                list1.push(first_col);
                list2.push(second_col);
            }
            _ => {
                eprintln!("ERROR")
            }
        }
    }

    let mut counter = CountingHashMap::new();
    for num in list2 {
        counter.increment(num);
    }
    let mut counts = vec![];
    for num in &list1 {
        let count = counter.get_count(num) as i32;
        counts.push(count * num);
    }
    let result: i32 = counts.iter().sum();    

    Ok(format!("{}", result))
}


/// A Python module implemented in Rust.
#[pymodule]
fn day1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve1, m)?)?;
    m.add_function(wrap_pyfunction!(solve2, m)?)?;
    Ok(())
}
