use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn solve1(inp: &str) -> PyResult<String> {
    let lines: Vec<&str> = inp.split('\n').collect();
    let mut safe_lines: Vec<&str> = vec![];
    for line in lines {
        let mut safe: bool = false;
        let nums: Vec<i32> = line
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if nums.is_sorted_by(|a, b| a < b) || nums.is_sorted_by(|a, b| b < a) {
            safe = true;
        }
        for i in 1..nums.len() {
            if (nums[i] - nums[i-1]) == 0 {
                safe = false;
            }
            if (nums[i] - nums[i-1]).abs() > 3 {
                safe = false;
            }
        }
        if !safe {
            continue
        }
        safe_lines.push(line);
    }

    Ok(format!("{}", safe_lines.len()))
}

fn test(nums: Vec<i32>) -> bool {
    let mut safe: bool = true;
    if !nums.is_sorted_by(|a, b| a < b) && !nums.is_sorted_by(|a, b| b < a) {
        safe = false;
    }
    for i in 1..nums.len() {
        if (nums[i] - nums[i-1]) == 0 {
            safe = false;
        }
        if (nums[i] - nums[i-1]).abs() > 3 {
            safe = false;
        }
    }
    safe
}

#[pyfunction]
fn solve2(inp: &str) -> PyResult<String> {
    let lines: Vec<&str> = inp.split('\n').collect();
    let mut safe_lines: Vec<&str> = vec![];
    for line in lines {
        let nums: Vec<i32> = line
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        
        if !test(nums.clone()) {
            let mut safe: bool = false;
            for i in 0..nums.len() {
                let mut new: Vec<i32> = nums.clone();
                new.remove(i);
                if test(new) {
                    safe = true;
                }
            }
            if !safe {
                continue
            }    
        }

        safe_lines.push(line);
    }

    Ok(format!("{}", safe_lines.len()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn day2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve1, m)?)?;
    m.add_function(wrap_pyfunction!(solve2, m)?)?;
    Ok(())
}
