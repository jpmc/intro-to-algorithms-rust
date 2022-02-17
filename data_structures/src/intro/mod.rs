// Euclid's `gcd(m,n) == gcd(n, m mod n)` definition
pub fn gcd_euclid(mut m: u32, mut n: u32) -> u32 {
    while n != 0 {
        let r = m % n;
        m = n;
        n = r;
    }
    m
}

// Consecutive integer checking algorithm for GCD. 
pub fn gcd_int_checking(m: u32, n: u32) -> u32 {
    let mut t = std::cmp::min(m, n);
    loop {
        if m % t == 0 {
            if n % t == 0 {
                return t;
            }
        }
        t -= 1;
    }
}

// Middle school "list the factors of each, and multiply the ones they share" method for GCD.
// Note: Requires a factoring method.
/*pub fn gcd_factoring(mut m: u32, mut n: u32) -> u32 {
    5
}*/

// Get all primes less than N. Just used in this example for `gcd_factoring`.
pub fn sieve_of_earosthenes(n: usize) -> Vec<usize> {
    // New vector of boolean flags, mark all entries as prime=true by default.
    let mut sieve = vec![true; n];
    // 0 and 1 will never be primes. Ever.
    sieve[0] = false;
    sieve[1] = false;

    // An empty vector to collect the integer primes that go with the flags as we go.
    let mut primes: Vec<usize> = Vec::new();

    //From 2 to N, start eliminating composite numbers.
    for i in 2..n {
        // If this isn't an already-eliminated composite, it's a prime!
        // Start eliminating multiples of it from the list.
        if sieve[i] {
            primes.push(i);
            let mut j: usize = 2*i;
            while j < (n as usize) {
                sieve[j] = false;
                j += i;
            }
        }
    }

    primes
}

// Exercises 1.1, question 4. Wants the integer square root.
// Done "by hand" instead of casting/using built-ins.
pub trait ISqrt<T> {
    fn isqrt(self) -> T;
}
// Macro to add an `.isqrt()` function to a given integer type.
// isqrt function uses bisection to iterate to the right result.
macro_rules! add_isqrt {
    ($t:ident) => {
        impl ISqrt<$t> for $t {
            fn isqrt(self) -> $t { 
                let mut hi = self;
                let mut lo = 0;
                let mut mid = (hi + lo) / 2;
                let mut mid_2 = mid * mid;
                while lo < hi - 1 && mid_2 != self {
                    if mid_2 < self { lo = mid; }
                    else { hi = mid; }
                    mid = (hi + lo) / 2;
                    mid_2 = mid*mid;
                }
                mid
             }
        }
    };
}
// *ALL* the integer types! \o/
add_isqrt!(usize); add_isqrt!(isize);
add_isqrt!(u8);    add_isqrt!(i8);
add_isqrt!(u16);   add_isqrt!(i16);
add_isqrt!(u32);   add_isqrt!(i32);
add_isqrt!(u64);   add_isqrt!(i64);
add_isqrt!(u128);  add_isqrt!(i128);

// Exercises 1.1, question 10. Extended Euclidean Algorithm.
// Takes inputs (m, n), and returns a tuple (x, y, d = gcd(m, n))
//   `d` is the standard GCD if `m` & `n`, as per the standard Euclid method. 
//   `x` and `y` are coefficients/solutions such that:  [ mx + ny = d ]
pub fn extended_euclid(m: i32, n: i32) -> (i32, i32, i32) {
    let (mut old_d, mut d) = (m, n);
    let (mut old_x, mut x) = (1, 0);
    let (mut old_y, mut y) = (0, 1);
    while d != 0 {
        /*
            Destructuring assignments are not yet stable. So `tmp` variable used.
            Github merged them in as stable, but isn't landed in rustc that I'm using.
        */
        let quotient = old_d / d;
        let mut tmp;

        // Standard "use remainder/mod to find GCD" step.
        tmp = d;
        d = old_d - quotient * d;
        old_d = tmp;
        // (old_d, d) = (d, old_d - quotient * d);
        
        // Keep track of coefficients `x` and `y` per iteration, 
        // rather than "backtracking" as when done by hand.
        tmp = x;
        x = old_x - quotient * x;
        old_x = tmp;
        // (old_x, x) = (x, old_x - quotient * x);
        tmp = y;
        y = old_y - quotient * y;
        old_y = tmp;
        // (old_y, y) = (y, old_y - quotient * y);
    }
    (old_x, old_y, old_d)
}


