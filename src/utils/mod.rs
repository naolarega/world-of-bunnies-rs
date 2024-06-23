pub mod simple_rng;

pub trait RandomNumberGenerator {
    fn next_u32(&mut self) -> u32;

    /// ## Generate in range
    /// includes the min
    fn gen_range(&mut self, min: u32, max: u32) -> u32;
}
