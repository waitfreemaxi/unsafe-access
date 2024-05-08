# `unsafe-access`

A collection of Rust procedural macros designed to simplify the use of unsafe array indexing while maintaining traditional indexing semantics. These macros are especially beneficial in performance-critical areas where bounds-checking can be explicitly bypassed, assuming prior validation. The key feature is retaining standard array indexing syntax while skipping bounds checks, which can yield significant performance benefits.

## Macros

- `unchecked_indices_ref`, `unchecked_indices_clone`, `unchecked_indices_copy`. These macros let the user access array elements via `get_unchecked` in a limited scope. This is `unsafe` code.

```rust
use unsafe_access::unchecked_indices_copy;

let array = [1, 2, 3, 4, 5, 6];
let indices = [0, 3, 5];

let copied_array: [i32; 3] = unchecked_indices_copy! {
    [
        array[indices[0]],
        array[indices[1]],
        array[indices[2]],
    ]
};
```

- `unsafe_access_fn`
An attribute macro that transforms standard array indexing within a function into unsafe, unchecked array indexing. This macro ensures that all indexing operations inside the function are converted to use `get_unchecked`. This example can be found in `examples/matrices.rs`:

```rust
use unsafe_access::unsafe_access_fn;

const MATRIX_SIZE: usize = 100;
const QUERY_SIZE: usize = 5;

fn main() {
    let matrix: [u64; MATRIX_SIZE * MATRIX_SIZE] = core::array::from_fn(|i| i as u64);
    let indices: [usize; QUERY_SIZE] = core::array::from_fn(|_| rand::random::<usize>() % (MATRIX_SIZE * MATRIX_SIZE));

    let result = unsafe { prechecked_indices(&matrix, &indices) };
    println!("result: {result:?}");
}

#[unsafe_access_fn]
unsafe fn prechecked_indices<T: Copy, const N: usize>(array: &[T], indices: &[usize; N]) -> [T; N] {
    core::array::from_fn(|i| array[indices[i]])
}
```

To run the example, execute:

```bash
cargo run --example matrices
```

## Benchmarks

For a simple benchmark using random accesses, see `benches/access.rs`, or via

```bash
cargo bench
```
