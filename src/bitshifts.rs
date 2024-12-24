/**
 * We divide the overall bit_index by usize::BITS (discarding the remainder) and use it as index to access the integer in integers vector. Then we use the remainder to access the bit in that integer.
 */
const BITSHIFTS_NEEDED: u8 = match usize::BITS {
    128 => 7,
    64 => 6,
    32 => 5,
    16 => 4,
    8 => 3,
    _ => panic!("Unsupported architecture"),
};

const REMAINDER_MASK: usize = (usize::MAX << (usize::BITS - BITSHIFTS_NEEDED as u32))
    >> (usize::BITS - BITSHIFTS_NEEDED as u32);

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use std::time::SystemTime;

    use crate::bitshifts::REMAINDER_MASK;

    #[test]
    fn test_sample() {
        println!("{}", 256 >> 7);
        println!("{}", 256 / 128);
        println!("{}", usize::BITS - BITSHIFTS_NEEDED as u32);
        println!("{:b}", REMAINDER_MASK);
    }

    /**
     * This test uses a fixed divisor for modulo calculation which can allow compiler to optimize the modulo to bit operations. The test result shows that compiler optimization averages to bit mask operation speed. 
     *  
     * 1000000000 iterations
     * ```
     * fixed bitmasks: 1458ms, modulos: 1616ms, 50
     * ```
     * 10000000000 iterations
     * ```
     * fixed bitmasks: 22760ms, modulos: 22630ms, 12
     * ```
     * 100000000000 iterations
     * ```
     * fixed bitmasks: 226285ms, modulos: 226640ms, 13
     * ```
     */
    #[test]
    fn test_bitmask_vs_modulo_fixed() {
        let mut numbers: Vec<usize> = Vec::new();
        for _i in 1..=10000 {
            numbers.push(random::<usize>());
        }

        // siphasher
        let mut a: usize = 0;
        let mut now = SystemTime::now();
        for _i in 1..100000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() & REMAINDER_MASK);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let bitmask_then = now.elapsed().unwrap().as_millis();

        now = SystemTime::now();
        for _i in 1..100000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() % usize::BITS as usize);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let modulo_then = now.elapsed().unwrap().as_millis();
        println!(
            "fixed bitmasks: {}ms, modulos: {}ms, {}",
            bitmask_then, modulo_then, a
        );
    }

    /**
     * This test uses a dynamic divisor for modulo calculation which can not allow compiler to optimize the modulo to bit operations. The test result shows that unoptimized modulo is definitely very slow compared to bitmask operation. 
     *
     * ```
     * bitmasks: 2286ms, modulos: 10598ms, 19
     * ```
     */
    #[test]
    fn test_bitmask_vs_modulo() {
        let mut numbers: Vec<usize> = Vec::new();
        for _i in 1..=10000 {
            numbers.push(random::<usize>());
        }
        let mut modulos: Vec<usize> = Vec::new();
        modulos.push(1);
        modulos.push(2);
        modulos.push(4);
        modulos.push(8);
        modulos.push(16);
        modulos.push(32);
        modulos.push(64);

        let mut bitmasks: Vec<usize> = Vec::new();
        bitmasks.push(0);
        bitmasks.push((usize::MAX << (usize::BITS - 1)) >> (usize::BITS - 1));
        bitmasks.push((usize::MAX << (usize::BITS - 2)) >> (usize::BITS - 2));
        bitmasks.push((usize::MAX << (usize::BITS - 3)) >> (usize::BITS - 3));
        bitmasks.push((usize::MAX << (usize::BITS - 4)) >> (usize::BITS - 4));
        bitmasks.push((usize::MAX << (usize::BITS - 5)) >> (usize::BITS - 5));
        bitmasks.push((usize::MAX << (usize::BITS - 6)) >> (usize::BITS - 6));

        // siphasher
        let mut a: usize = 0;
        let mut now = SystemTime::now();
        for _i in 1..10000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() & bitmasks[_i % 7]);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let bitmask_then = now.elapsed().unwrap().as_millis();

        now = SystemTime::now();
        for _i in 1..10000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() % modulos[_i % 7]);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let modulo_then = now.elapsed().unwrap().as_millis();
        println!(
            "bitmasks: {}ms, modulos: {}ms, {}",
            bitmask_then, modulo_then, a
        );
    }

    /**
     * Based on the test results, it seems that the compiler optimizes divisions using bitshifts.
     * ------
     * bitshifts: 1432ms, divisions: 1437ms, 10809776988295580
     * ------
     */
    #[test]
    fn test_bitshifts_vs_divisions() {
        let mut numbers: Vec<usize> = Vec::new();
        for _i in 1..=10000 {
            numbers.push(random::<usize>());
        }
        let mut divisors: Vec<usize> = Vec::new();
        divisors.push(1);
        divisors.push(2);
        divisors.push(4);
        divisors.push(8);
        divisors.push(16);
        divisors.push(32);
        divisors.push(64);

        let mut bitshifters: Vec<usize> = Vec::new();
        bitshifters.push(0);
        bitshifters.push(1);
        bitshifters.push(2);
        bitshifters.push(3);
        bitshifters.push(4);
        bitshifters.push(5);
        bitshifters.push(6);

        // siphasher
        let mut a: usize = 0;
        let mut now = SystemTime::now();
        for _i in 1..1000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() >> bitshifters[_i % 7]);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let bitshift_then = now.elapsed().unwrap().as_millis();

        now = SystemTime::now();
        for _i in 1..1000000000 {
            a = a.wrapping_add(numbers[_i % 10000].clone() / divisors[_i % 7]);
            if _i % 2 == 0 {
                a = 0;
            }
        }
        let divisor_then = now.elapsed().unwrap().as_millis();
        println!(
            "bitshifts: {}ms, divisions: {}ms, {}",
            bitshift_then, divisor_then, a
        );
    }
}
