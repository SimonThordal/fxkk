use std::vec;

use pyo3::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
use smallvec::{SmallVec, smallvec};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn levenshtein_mat(a: &str, b: &str) -> PyResult<usize> {
    if a == b {
        return Ok(0)
    }
    let a = UnicodeSegmentation::graphemes(a, true).collect::<Vec<&str>>();
    let b = UnicodeSegmentation::graphemes(b, true).collect::<Vec<&str>>();
    let l_a: usize = a.len() + 1;
    let l_b: usize = b.len() + 1;
    let mut D: Vec<Vec<usize>> = vec![];
    for _ in 0..l_a {
        D.push(vec![0; l_b])
    }
    for i in 1..l_a {
        D[i][0] = i;
    }
    for j in 1..l_b {
        D[0][j] = j;
    }
    let mut cost: usize;
    for i in 1..l_a {
        let a_i = a[i -1];
        for j in 1..l_b {
            let b_j = b[j - 1];
            cost = if a_i == b_j { 0 } else { 1 };
            D[i][j] = std::cmp::min(
                D[i - 1][j] + 1,
                std::cmp::min(
                    D[i][j - 1] + 1, 
                    D[i - 1][j - 1] + cost
                ),
            );
        }
    }
    Ok(D[l_a - 1][l_b - 1])
}

#[pyfunction]
fn levenshtein_vec(a: &str, b: &str) -> PyResult<usize> {
    if a == b {
        return Ok(0)
    }
    let a: Vec<&str> = UnicodeSegmentation::graphemes(a, true).collect::<Vec<&str>>();
    let b: Vec<&str> = UnicodeSegmentation::graphemes(b, true).collect::<Vec<&str>>();
    let l_a: usize = a.len() + 1;
    let l_b: usize = b.len() + 1;
    let mut v_0: Vec<usize> = vec![0; l_b];
    let mut v_1: Vec<usize> = (0..l_b).collect();
    for i in 1..l_a {
        v_0 = v_1;
        v_1 =vec![0; l_b];
        v_1[0] = i;
        for j in 1..l_b {
            let cost: usize = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            v_1[j] = std::cmp::min(
                v_0[j] + 1,
                std::cmp::min(
                    v_1[j - 1] + 1,
                    v_0[j - 1] + cost
                )
            );
        }
    }
    Ok(v_1[l_b - 1])
}

#[pyfunction]
fn levenshtein_exp(a: &str, b: &str) -> PyResult<usize> {
    if a.len() > b.len() {
        return levenshtein_exp(b, a);
    }
    if a.len() == 0 {
        return Ok(b.len());
    }
    if b.len() == 0 {
        return Ok(a.len());
    }
    if a == b {
        return Ok(0)
    }
    
    let l_b: usize = b.len() + 1;
    
    let mut row: Vec<usize> = (1..l_b).collect();
    let mut previous_diagonal: usize;
    let mut cost: usize;
    let mut previous_row: usize = 0;
    for (i, c_a) in a.chars().enumerate() {
        previous_row = i +1;
        let mut previous_above = i;
        for (j, c_b) in b.chars().enumerate() {
            cost = if c_a == c_b { 0 } else { 1 };
            previous_diagonal = previous_above;
            previous_above = row[j];
            previous_row = std::cmp::min(previous_diagonal+cost, // Substitution 
                std::cmp::min(
                    previous_above+1, // Deletion
                    previous_row+1 // Insertion
                )
            );
            row[j] = previous_row;
        }
    } 

    Ok(previous_row)
    
}

#[pyfunction]
pub fn levenshtein_exp_theirs(a: &str, b: &str) -> PyResult<usize>
{
    if a.len() > b.len() {
        return levenshtein_exp(b, a);
    }
    if a.len() == 0 {
        return Ok(b.len());
    }
    if b.len() == 0 {
        return Ok(a.len());
    }
    if a == b {
        return Ok(0)
    }

    let b_len = b.chars().count();

    let mut cache: Vec<usize> = (1..b_len + 1).collect();

    let mut result = b_len;

    for (i, a_elem) in a.chars().enumerate() {
        result = i + 1;
        let mut distance_b = i;

        for (j, b_elem) in b.chars().enumerate() {
            let cost = usize::from(a_elem != b_elem);
            let distance_a = distance_b + cost;
            distance_b = cache[j];
            result = std::cmp::min(result + 1, std::cmp::min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn fast_levenshtein(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein_mat, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein_vec, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein_exp, m)?)?;
    Ok(())
}


