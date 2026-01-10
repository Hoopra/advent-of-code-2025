pub fn find_min_max(a: usize, b: usize) -> (usize, usize) {
    match a > b {
        true => (b, a),
        _ => (a, b),
    }
}
