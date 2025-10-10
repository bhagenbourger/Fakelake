pub fn get_random_bool(probability: f64) -> bool {
    let random_number: f64 = crate::rng::f64();
    return random_number < probability;
}