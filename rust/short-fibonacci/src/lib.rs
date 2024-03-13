/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u64> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
fn fib_generic() -> Vec<u64> {
    let mut vec = create_buffer(5);
    let mut tmp1 = 1;
    let mut tmp2 = 1;
    for (i, elem) in vec.iter_mut().enumerate() {
        if i == 0 || i == 1 {
            *elem = 1;
        } else {
            *elem = tmp1 + tmp2;
            tmp1 = tmp2;
            tmp2 = *elem;
        }
    }
    return vec;
}
pub fn fibonacci() -> Vec<u8> {
    fib_generic().iter().map(|&x| x as u8).collect()
}
