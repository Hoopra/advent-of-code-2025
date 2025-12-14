pub fn divide_integer(integer: usize, divisor: f64) -> usize {
    ((integer as f64) / divisor).floor() as usize
}
