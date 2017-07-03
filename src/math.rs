#![allow(dead_code)]
#[derive(Debug)]
pub struct MathError {
    what: String
}

impl MathError {
    fn new(msg: &str) -> MathError {
        MathError { what: msg.to_string() }
    }
}

// we extend std::result::Result into a custom type
pub type Result<T> = ::std::result::Result<T, MathError>;

/// vector dot
pub fn dot(a: &[f32], b: &[f32]) -> Result<f32> {
    let mut sum = 0.0;

    if a.len() != b.len() {
        return Err(MathError::new("length mismatch"))
    } else {
        for i in 0 .. a.len() { // could use some `iterator.map.collect` thing but eh
            sum += a[i] * b[i]
        }
    }
    Ok(sum)
}

/// vector subtraction
pub fn diff(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    let mut diff = Vec::new();

    if a.len() != b.len() {
        return Err(MathError::new("length mismatch"))
    } else {
        for i in 0 .. a.len() {
            diff.push(a[i] - b[i])
        }
    }
    Ok(diff)
}

/// vector summation
pub fn sum(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    let mut sum = Vec::new();

    if a.len() != b.len() {
        return Err(MathError::new("length mismatch"))
    } else {
        for i in 0 .. a.len() {
            sum.push(a[i] + b[i])
        }
    }
    Ok(sum)
}
