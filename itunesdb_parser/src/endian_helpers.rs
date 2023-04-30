pub mod endian_helpers {

    // TODO: Build version that can handle endianness
    pub fn build_integer_from_bytes(bytes : &[u8]) -> u32 {

        let mut number: u32 = 0;
        const RADIX : u32 = 10;

        for (idx, item) in bytes.iter().enumerate() {

            let summand : u32 = RADIX.checked_pow(idx as u32).unwrap_or_else(|| panic!("Can't raise {} to power {}", RADIX, idx)) as u32;

            number += summand as u32 + *item as u32;
        }

        return number;
    }

}