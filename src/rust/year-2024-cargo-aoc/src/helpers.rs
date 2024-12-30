#[allow(unused)]
pub(crate) fn transpose<T: Copy + Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.iter().map(|n| n.iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .cloned()
                .collect::<Vec<T>>()
        })
        .collect()
}
