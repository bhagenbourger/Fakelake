use fastrand::Rng;
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Option<Rng>> = const { RefCell::new(None) };
}

/// Initialize the global RNG with an optional seed
/// If seed is None, uses a random seed
pub fn initialize_rng(seed: Option<u64>) {
    RNG.with(|rng| {
        let mut rng_ref = rng.borrow_mut();
        *rng_ref = Some(match seed {
            Some(s) => Rng::with_seed(s),
            None => Rng::new(),
        });
    });
}

/// Execute a function with access to the global RNG
/// Auto-initializes with a random seed if not already initialized
pub fn with_rng<T>(f: impl FnOnce(&mut Rng) -> T) -> T {
    RNG.with(|rng| {
        let mut rng_ref = rng.borrow_mut();
        if rng_ref.is_none() {
            // Auto-initialize with random seed if not already done
            *rng_ref = Some(Rng::new());
        }
        let rng = rng_ref.as_mut().unwrap();
        f(rng)
    })
}

/// Generate a random boolean
pub fn bool() -> bool {
    with_rng(|rng| rng.bool())
}

/// Generate a random f64 in range [0.0, 1.0)
pub fn f64() -> f64 {
    with_rng(|rng| rng.f64())
}

/// Generate a random i32 in the given range
pub fn i32(range: std::ops::Range<i32>) -> i32 {
    with_rng(|rng| rng.i32(range))
}

/// Generate a random i64 in the given range
pub fn i64(range: std::ops::Range<i64>) -> i64 {
    with_rng(|rng| rng.i64(range))
}

/// Generate a random u32 in the given range
pub fn u32(range: std::ops::Range<u32>) -> u32 {
    with_rng(|rng| rng.u32(range))
}

/// Generate a random usize in the given range  
pub fn usize(range: std::ops::RangeTo<usize>) -> usize {
    with_rng(|rng| rng.usize(range))
}

/// Generate a random alphanumeric character
pub fn alphanumeric() -> char {
    with_rng(|rng| rng.alphanumeric())
}

/// Generate a random f64 in the given range using fastrand_contrib
pub fn f64_range(range: std::ops::Range<f64>) -> f64 {
    with_rng(|rng| {
        // Use the same approach as fastrand_contrib::f64_range but with our RNG
        let start = range.start;
        let end = range.end;
        start + rng.f64() * (end - start)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_with_seed() {
        // Test that same seed produces same results
        initialize_rng(Some(12345));
        let results1: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        initialize_rng(Some(12345));
        let results2: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        assert_eq!(results1, results2);
    }

    #[test]
    fn test_different_seeds_produce_different_results() {
        initialize_rng(Some(12345));
        let results1: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        initialize_rng(Some(54321));
        let results2: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        assert_ne!(results1, results2);
    }

    #[test]
    fn test_no_seed_works() {
        initialize_rng(None);
        let result = i32(0..100);
        assert!((0..100).contains(&result));
    }
}
