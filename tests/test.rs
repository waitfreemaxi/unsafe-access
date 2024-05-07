use unsafe_access::{unchecked_indices_copy, unchecked_indices_ref};

#[test]
fn array_access() {
    let [a, b, c, d, e, f] = rand::random::<[usize; 6]>().map(|i| i % 6);

    let foo = rand::random::<[u64; 6]>();

    let _ref_access: [&u64; 6] = unchecked_indices_ref! {
        [
            foo[a], foo[b], foo[c],
            foo[d], foo[e], foo[f],
        ]

    };

    let _copy_access: [u64; 6] = unchecked_indices_copy! {
        [
            foo[a], foo[b], foo[c],
            foo[d], foo[e], foo[f],
        ]
    };
}

#[test]
fn array_access_in_function() {
    use unsafe_access::unsafe_access_fn;

    #[unsafe_access_fn]
    pub unsafe fn access_elements(slice: &[i32]) -> i32 {
        let first = slice[0];
        let second = slice[1];
        first + second
    }

    let x = unsafe { access_elements(&[1, 2]) };
    assert_eq!(x, 3)
}

#[test]
fn handle_nested_indices() {
    use unsafe_access::unsafe_access_fn;

    #[unsafe_access_fn]
    pub unsafe fn access_elements(slice: &[usize]) -> usize {
        slice[slice[0]]
    }

    let x = unsafe { access_elements(&[1, 0]) };
    // 0 -> 1 -> 0
    assert_eq!(x, 0)
}
