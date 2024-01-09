// lib.rs
use rand::{seq::SliceRandom, RngCore};

struct FillAdaptor<'a>(&'a [u8]);

impl<'a> rand::RngCore for FillAdaptor<'a> {
    fn next_u32(&mut self) -> u32 {
        let (value, rest) = self.0.split_at(std::mem::size_of::<u32>());
        self.0 = rest;
        u32::from_ne_bytes(value.try_into().unwrap())
    }

    fn next_u64(&mut self) -> u64 {
        let (value, rest) = self.0.split_at(std::mem::size_of::<u64>());
        self.0 = rest;
        u64::from_ne_bytes(value.try_into().unwrap())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let (bytes, rest) = self.0.split_at(dest.len());
        dest.copy_from_slice(bytes);
        self.0 = rest;
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

// Define a trait for randomization
trait Randomizer {
    fn randomize_in_place<T>(&mut self, array: &mut [T]);
    fn randomize_new_array<T: Clone>(&mut self, array: &[T]) -> Vec<T>;
}

// Implement the trait for the default rand::Rng
impl Randomizer for rand::rngs::ThreadRng {
    fn randomize_in_place<T>(&mut self, array: &mut [T]) {
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
    }

    fn randomize_new_array<T: Clone>(&mut self, array: &[T]) -> Vec<T> {
        let mut rng = rand::thread_rng();
        let mut randomized = array.to_vec();
        randomized.shuffle(&mut rng);
        randomized
    }
}

// Implement the trait for the cryptographic rand_core::OsRng
impl Randomizer for rand_core::OsRng {
    fn randomize_in_place<T>(&mut self, array: &mut [T]) {
        let mut buffer = [0u8; 32]; // Adjust the size as needed
        self.fill_bytes(&mut buffer);
        let mut rng = FillAdaptor(&buffer);
        array.shuffle(&mut rng);
    }

    fn randomize_new_array<T: Clone>(&mut self, array: &[T]) -> Vec<T> {
        let mut buffer = vec![0u8; 32]; // Adjust the size as needed
        self.fill_bytes(&mut buffer);
        let mut rng = FillAdaptor(&buffer);
        let mut randomized = array.to_vec();
        randomized.shuffle(&mut rng);
        randomized
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomize_in_place() {
        let mut rng = rand::thread_rng();
        test_randomization(&mut rng);
    }

    #[test]
    fn test_randomize_new_array() {
        let mut rng = rand::thread_rng();
        test_randomization(&mut rng);
    }

    #[test]
    fn test_crypto_randomize_in_place() {
        let mut rng = rand_core::OsRng::default();
        test_randomization(&mut rng);
    }

    #[test]
    fn test_crypto_randomize_new_array() {
        let mut rng = rand_core::OsRng::default();
        test_randomization(&mut rng);
    }

    fn test_randomization<R: Randomizer>(rng: &mut R) {
        let mut original_array = vec![1, 2, 3, 4, 5];
        let mut array = original_array.clone();

        // Display the original array for reference
        println!("Original Array: {:?}", original_array);

        rng.randomize_in_place(&mut array);

        // Display the randomized array for reference
        println!("Randomized Array: {:?}", array);

        // Make assertions about the randomized array
        assert_eq!(array.len(), original_array.len());
        assert_eq!(array.sort(), original_array.sort()); // Sort for comparison
    }
}
