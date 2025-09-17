/// Generates a vector of evenly spaced integers between start and end (exclusive).
pub fn linspace(start: i64, end: i64, num: i64) -> Vec<i64> {
    let steps: i64 = (end - start)/num;
    let mut linspaced_vec: Vec<i64> = vec![start];

    for i in 0..num {
        if i != 0 {
            linspaced_vec.push(start + i * steps);
        }
    }

    // Return the final vector of evenly spaced values
    linspaced_vec
}

/// Creates a 2D vector (matrix) filled with zeros of given size (rows, columns).
pub fn zeros(size: (i64, i64)) -> Vec<Vec<i64>> {
    // Initialize an empty outer vector to hold rows
    let mut zeros: Vec<Vec<i64>> = Vec::new();
    
    for _ in 0..size.0 {
        // Create a row of `size.1` zeros and push into the matrix
        zeros.push(vec![0; size.1 as usize]);
    }

    // Return the completed 2D vector (matrix) of zeros
    zeros
}