use criterion::{criterion_group, criterion_main, Criterion};
use unsafe_access::unsafe_access_fn;

const MATRIX_SIZE: usize = 10;

fn access_benchmark(c: &mut Criterion) {
    let matrix: Box<[u64; MATRIX_SIZE * MATRIX_SIZE]> = Box::new([0; MATRIX_SIZE * MATRIX_SIZE]);
    let indices: [usize; 100] =
        [(); 100].map(|_| rand::random::<usize>() % MATRIX_SIZE * MATRIX_SIZE);

    c.bench_function("Unchecked Copy Access", |b| {
        b.iter(
            #[inline]
            || unsafe { unsafe_matrix_access(&matrix, &indices) },
        )
    });

    c.bench_function("Safe Copy Access", |b| {
        b.iter(
            #[inline]
            || unsafe { safe_matrix_access(&matrix, &indices) },
        )
    });
}

#[rustfmt::skip]
#[unsafe_access_fn]
unsafe fn unsafe_matrix_access(
    matrix: &[u64; MATRIX_SIZE * MATRIX_SIZE],
    indices: &[usize]
) -> [u64; 100] {
    [
        matrix[indices[0]], matrix[indices[1]], matrix[indices[2]], matrix[indices[3]], matrix[indices[4]],
        matrix[indices[5]], matrix[indices[6]], matrix[indices[7]], matrix[indices[8]], matrix[indices[9]],
        matrix[indices[10]], matrix[indices[11]], matrix[indices[12]], matrix[indices[13]], matrix[indices[14]],
        matrix[indices[15]], matrix[indices[16]], matrix[indices[17]], matrix[indices[18]], matrix[indices[19]],
        matrix[indices[20]], matrix[indices[21]], matrix[indices[22]], matrix[indices[23]], matrix[indices[24]],
        matrix[indices[25]], matrix[indices[26]], matrix[indices[27]], matrix[indices[28]], matrix[indices[29]],
        matrix[indices[30]], matrix[indices[31]], matrix[indices[32]], matrix[indices[33]], matrix[indices[34]],
        matrix[indices[35]], matrix[indices[36]], matrix[indices[37]], matrix[indices[38]], matrix[indices[39]],
        matrix[indices[40]], matrix[indices[41]], matrix[indices[42]], matrix[indices[43]], matrix[indices[44]],
        matrix[indices[45]], matrix[indices[46]], matrix[indices[47]], matrix[indices[48]], matrix[indices[49]],
        matrix[indices[50]], matrix[indices[51]], matrix[indices[52]], matrix[indices[53]], matrix[indices[54]],
        matrix[indices[55]], matrix[indices[56]], matrix[indices[57]], matrix[indices[58]], matrix[indices[59]],
        matrix[indices[60]], matrix[indices[61]], matrix[indices[62]], matrix[indices[63]], matrix[indices[64]],
        matrix[indices[65]], matrix[indices[66]], matrix[indices[67]], matrix[indices[68]], matrix[indices[69]],
        matrix[indices[70]], matrix[indices[71]], matrix[indices[72]], matrix[indices[73]], matrix[indices[74]],
        matrix[indices[75]], matrix[indices[76]], matrix[indices[77]], matrix[indices[78]], matrix[indices[79]],
        matrix[indices[80]], matrix[indices[81]], matrix[indices[82]], matrix[indices[83]], matrix[indices[84]],
        matrix[indices[85]], matrix[indices[86]], matrix[indices[87]], matrix[indices[88]], matrix[indices[89]],
        matrix[indices[90]], matrix[indices[91]], matrix[indices[92]], matrix[indices[93]], matrix[indices[94]],
        matrix[indices[95]], matrix[indices[96]], matrix[indices[97]], matrix[indices[98]], matrix[indices[99]],
    ]
}

#[rustfmt::skip]
unsafe fn safe_matrix_access(
    matrix: &[u64; MATRIX_SIZE * MATRIX_SIZE],
    indices: &[usize]
) -> [u64; 100] {
    [
        matrix[indices[0]], matrix[indices[1]], matrix[indices[2]], matrix[indices[3]], matrix[indices[4]],
        matrix[indices[5]], matrix[indices[6]], matrix[indices[7]], matrix[indices[8]], matrix[indices[9]],
        matrix[indices[10]], matrix[indices[11]], matrix[indices[12]], matrix[indices[13]], matrix[indices[14]],
        matrix[indices[15]], matrix[indices[16]], matrix[indices[17]], matrix[indices[18]], matrix[indices[19]],
        matrix[indices[20]], matrix[indices[21]], matrix[indices[22]], matrix[indices[23]], matrix[indices[24]],
        matrix[indices[25]], matrix[indices[26]], matrix[indices[27]], matrix[indices[28]], matrix[indices[29]],
        matrix[indices[30]], matrix[indices[31]], matrix[indices[32]], matrix[indices[33]], matrix[indices[34]],
        matrix[indices[35]], matrix[indices[36]], matrix[indices[37]], matrix[indices[38]], matrix[indices[39]],
        matrix[indices[40]], matrix[indices[41]], matrix[indices[42]], matrix[indices[43]], matrix[indices[44]],
        matrix[indices[45]], matrix[indices[46]], matrix[indices[47]], matrix[indices[48]], matrix[indices[49]],
        matrix[indices[50]], matrix[indices[51]], matrix[indices[52]], matrix[indices[53]], matrix[indices[54]],
        matrix[indices[55]], matrix[indices[56]], matrix[indices[57]], matrix[indices[58]], matrix[indices[59]],
        matrix[indices[60]], matrix[indices[61]], matrix[indices[62]], matrix[indices[63]], matrix[indices[64]],
        matrix[indices[65]], matrix[indices[66]], matrix[indices[67]], matrix[indices[68]], matrix[indices[69]],
        matrix[indices[70]], matrix[indices[71]], matrix[indices[72]], matrix[indices[73]], matrix[indices[74]],
        matrix[indices[75]], matrix[indices[76]], matrix[indices[77]], matrix[indices[78]], matrix[indices[79]],
        matrix[indices[80]], matrix[indices[81]], matrix[indices[82]], matrix[indices[83]], matrix[indices[84]],
        matrix[indices[85]], matrix[indices[86]], matrix[indices[87]], matrix[indices[88]], matrix[indices[89]],
        matrix[indices[90]], matrix[indices[91]], matrix[indices[92]], matrix[indices[93]], matrix[indices[94]],
        matrix[indices[95]], matrix[indices[96]], matrix[indices[97]], matrix[indices[98]], matrix[indices[99]],
    ]
}

criterion_group!(benches, access_benchmark);
criterion_main!(benches);
