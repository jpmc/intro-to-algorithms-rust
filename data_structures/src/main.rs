
// Later, one day: https://www.reddit.com/r/rust/comments/q8wfbq/making_slow_rust_code_fast/
// Also later: Try writing `rot13` and comparing to `dd`/`cat` one day.
// -- https://www.reddit.com/r/programming/comments/qafnsk/casey_muratori_refterm_and_the_philosophy_of/hh3kt1f/
fn main() {
    println!("Hello, world!");
    intro_test();
}

mod intro;
fn intro_test() {
    // Test the various GCD methods, and ancillary methods.
    println!("E: {}", intro::gcd_euclid(60, 24));
    println!("I: {}", intro::gcd_int_checking(60, 24));
    println!("V: {:?}", intro::sieve_of_earosthenes(60));

    // Integer Square root exercise.
    use intro::ISqrt;
    println!("{}", (73 as u32).isqrt());

    // Extended Euclidean Algo Exercise 1.1
    let (x, y, d) = intro::extended_euclid(888, 54);
    println!("{:?}({:?}) + {:?}({:?}) = {:?}", 888, x, 54, y, d);
    // expected values (888, 54) -> (-2, 33, 6)

    let (x, y, d) = intro::extended_euclid(1180, 482);
    println!("{:?}({:?}) + {:?}({:?}) = {:?}", 1180, x, 482, y, d);
    // expected values (1180, 482) -> (-29, 71, 2)

    
}
