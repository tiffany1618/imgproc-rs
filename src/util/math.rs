use crate::util::Number;

// Multiplies a square matrix by a vector
// mat MUST be square; output vec has same dimensions as input vec
pub fn vector_mul<T: Number>(mat: &[T], vec: &[T]) -> Option<Vec<T>> {
    let rows = vec.len();
    let mat_cols = mat.len() / rows;

    if mat_cols != rows {
        return None;
    }

    let mut output = vec![0.into(); rows];

    for i in 0..rows {
        for j in 0..rows {
            output[i] += mat[rows * i + j] * vec[j];
        }
    }

    Some(output)
}