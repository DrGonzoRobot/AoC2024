use pyo3::prelude::*;
use regex::Regex;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn solve1(inp: &str) -> PyResult<String> {
    let mut xmas: i32 = 0;
    let re = Regex::new(r"(XMAS)").unwrap();

    let lines: Vec<&str> = inp.split('\n').collect();
    for line in lines {
        for (_, [_m]) in re.captures_iter(line).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&line.chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
    }
    //println!("Rows {}", xmas);
    let lines: Vec<&str> = inp.split('\n').collect();
    let mut i: usize = lines[0].len() - 1;
    while i >= 0 {
        let mut col = Vec::<&str>::new();
        for line in &lines {
            col.push(&line[i..i + 1]);
        }
        for (_, [_m]) in re.captures_iter(col.join("").as_str()).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&col.join("").as_str().chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
        if i > 0 {
            i -= 1;
        } else {
            break;
        };
    }
    //println!("Cols {}", xmas);



    let lines: Vec<&str> = inp.split('\n').collect();
    let imax = lines[0].len() - 4;
    let mut offset = 0;
    for start in 0..=imax {
        let mut diag = Vec::<&str>::new();
        for line in &lines {
            let i = start+offset;
            if i < line.len() {
                diag.push(&line[i..i+1]);
                offset += 1;
            }
        }
        //println!("{}", diag.join("").as_str());
        for (_, [_m]) in re.captures_iter(diag.join("").as_str()).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&diag.join("").as_str().chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
        offset = 0;
    }

    let lines: Vec<&str> = inp.split('\n').collect();
    let imax = lines[0].len() - 4;
    let mut offset = 0;
    for start in 0..=imax {
        let mut diag = Vec::<&str>::new();
        for line in lines.iter().rev() {
            let i = start+offset;
            if i < line.len() {
                diag.push(&line[i..i+1]);
                offset += 1;
            }
        }
        //println!("{}", diag.join("").as_str());
        for (_, [_m]) in re.captures_iter(diag.join("").as_str()).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&diag.join("").as_str().chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
        offset = 0;
    }

    let rinp = inp.chars().rev().collect::<String>();
    let lines: Vec<&str> = rinp.split('\n').collect();
    let imax = lines[0].len() - 4;
    let mut offset = 0;
    for start in 1..=imax {
        let mut diag = Vec::<&str>::new();
        for line in &lines {
            let i = start+offset;
            if i < line.len() {
                diag.push(&line[i..i+1]);
                offset += 1;
            }
        }
        //println!("{}", diag.join("").as_str());
        for (_, [_m]) in re.captures_iter(diag.join("").as_str()).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&diag.join("").as_str().chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
        offset = 0;
    }

    let rinp = inp.chars().rev().collect::<String>();
    let lines: Vec<&str> = rinp.split('\n').collect();
    let imax = lines[0].len() - 4;
    let mut offset = 0;
    for start in 1..=imax {
        let mut diag = Vec::<&str>::new();
        for line in lines.iter().rev() {
            let i = start+offset;
            if i < line.len() {
                diag.push(&line[i..i+1]);
                offset += 1;
            }
        }
        //println!("{}", diag.join("").as_str());
        for (_, [_m]) in re.captures_iter(diag.join("").as_str()).map(|c| c.extract()) {
            xmas += 1;
        }
        for (_, [_m]) in re
            .captures_iter(&diag.join("").as_str().chars().rev().collect::<String>())
            .map(|c| c.extract())
        {
            xmas += 1;
        }
        offset = 0;
    }

    

    Ok(format!("Hits {}", xmas))
}

#[pyfunction]
fn solve2(inp: &str) -> PyResult<String> {
    let mut xmas: i32 = 0;
    let lines: Vec<&str> = inp.split('\n').collect();
    let line_len = lines[0].len();
    let inp: &str = &lines.join("");
    println!("line len {} = {}", line_len, inp.len()/line_len);
    let imax = inp.len()-1-(line_len*2)-3;
    println!("imax {}", imax);

    let mut good: Vec<&str> = vec![];
    for i in 0..=imax {
        if !(i > 0 && (i % line_len == line_len-2 || i % line_len == line_len-1)) {
            println!("{}", i);
            let mut letters = Vec::<&str>::new();
            letters.push(&inp[i..i+1]);
            letters.push(&inp[i+2..i+3]);
            letters.push(&inp[i+line_len+1..i+line_len+2]);
            letters.push(&inp[i+(line_len*2)..i+(line_len*2)+1]);
            letters.push(&inp[i+(line_len*2)+2..i+(line_len*2)+3]);
            if letters[2] == "A" && !(letters.contains(&"X")){
                let test = vec![letters[0], letters[1], letters[3], letters[4]];
                if !test.contains(&"A") {
                    if letters[0] != letters[4] && letters[1] != letters[3] {
                        good.extend(letters.clone());
                        println!("{}", letters.join("").as_str());
                        xmas += 1;
                    }
                }
            }
        }
    }


    Ok(format!("Hits {}", xmas))
}

/// A Python module implemented in Rust.
#[pymodule]
fn day4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve1, m)?)?;
    m.add_function(wrap_pyfunction!(solve2, m)?)?;
    Ok(())
}
