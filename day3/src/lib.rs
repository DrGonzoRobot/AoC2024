use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
fn solve1(inp: &str) -> PyResult<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = vec![];
    for (_, [first, sec]) in re.captures_iter(inp).map(|c| c.extract()) {
        results.push(first.parse::<i32>()? * sec.parse::<i32>()?);
    }
    let result: i32 = results.iter().sum();
    Ok(format!("Matches: {}", result))
}

#[pyfunction]
fn solve2(inp: &str) -> PyResult<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut dflag: bool = true;
    let mut results = vec![];
    let mut i: usize = 0;
    let mut j: usize = 4;

    while i <= inp.len()-8 {

        if inp[i..j] == *"mul(" && dflag {
            while inp[j-1..j] != *")" && j <= i + 12 {
                j += 1;
            }
            for (_, [first, sec]) in re.captures_iter(&inp[i..j]).map(|c| c.extract()) {
                results.push(first.parse::<i32>()? * sec.parse::<i32>()?);
            }
            j = i + 4;
        }

        if inp[i..j] == *"do()" {
            dflag = true;
        }

        if inp[i..j+3] == *"don't()" {
            dflag = false;
        }

        i += 1;
        j += 1;

    }
    let result: i32 = results.iter().sum();
    Ok(format!("Matches: {}", result))
}

/// A Python module implemented in Rust.
#[pymodule]
fn day3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve1, m)?)?;
    m.add_function(wrap_pyfunction!(solve2, m)?)?;
    Ok(())
}
