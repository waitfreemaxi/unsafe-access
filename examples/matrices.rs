use unsafe_access::unsafe_access_fn;

const MATRIX_SIZE: usize = 100;
const QUERY_SIZE: usize = 5;

fn main() {
    // Matrix containing values 0..MATRIX_SIZE * MATRIX_SIZE - 1
    let matrix: [u64; MATRIX_SIZE * MATRIX_SIZE] = core::array::from_fn(|i| i as u64);

    // Indices to query. These are prechecked due to modulo
    let indices: [usize; QUERY_SIZE] =
        core::array::from_fn(|_| rand::random::<usize>() % MATRIX_SIZE * MATRIX_SIZE);

    // Fetch and print
    let result = unsafe { prechecked_indices(&matrix, &indices) };
    println!("result: {result:?}");
}

#[unsafe_access_fn]
unsafe fn prechecked_indices<T: Copy, const N: usize>(array: &[T], indices: &[usize; N]) -> [T; N] {
    core::array::from_fn(|i| array[indices[i]])
}
