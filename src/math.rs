#[allow(dead_code)]
#[derive(Debug)]
pub struct MathError {
    what: String
}

impl MathError {
    fn new(msg: &str) -> MathError {
        MathError { what: msg.to_string() }
    }
}

pub type Result<T> = ::std::result::Result<T, MathError>;

#[allow(dead_code)]
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

#[allow(dead_code)]
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
