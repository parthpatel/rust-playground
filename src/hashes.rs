/**
 * We divide the overall bit_index by usize::BITS (discarding the remainder) and use it as index to access the integer in integers vector. Then we use the remainder to access the bit in that integer. 
 */
const BITSHIFTS_NEEDED: u8 = match usize::BITS {
    128 => 6,
    64 => 5,
    32 => 4,
    16 => 3,
    8 => 4,
    _ => panic!("Unsupported architecture"),
};


#[cfg(test)]
mod tests {
    use std::hash::Hasher;
    use xxhash_rust::xxh3::xxh3_64_with_seed;
    use xxhash_rust::xxh3::Xxh3;

    #[test]
    fn test_hash_speeds() {
        println!(
            "| datalen | repetitions| sip_hasher | xxhash3-rust oneshot | xxhash3-rust streaming | twoxhash oneshot |"
        );
        { 1..256 }.for_each(|x| {
            test_hash_speeds_per_size(x);
        });

        [512, 1024, 2048].iter().for_each(|x| {
            test_hash_speeds_per_size(*x);
        });
    }

    fn test_hash_speeds_per_size(size: i32) {
        let repetitions = 10000000;
        // < 128 bytes
        let mut samples127: Vec<Vec<u8>> = Vec::new();
        { 1..11 }.for_each(|_| {
            let mut temp: Vec<u8> = Vec::new();
            { 1..size }.into_iter().for_each(|_| {
                temp.push(rand::random());
            });
            samples127.push(temp);
        });

        test_hash_speed(samples127, repetitions);
    }

    fn test_hash_speed(data: Vec<Vec<u8>>, total: usize) {
        let seed = 12897;
        let seed2 = 13297;
        let mut xxh3_hasher: Xxh3 = Xxh3::with_seed(seed);
        let mut sip_hasher = std::hash::SipHasher::new_with_keys(seed, seed2);

        let data_len = data[0].len();
        let mut new_data: Vec<&[u8]> = Vec::new();
        data.iter().for_each(|x| {
            new_data.push(x.as_slice());
        });

        let mut a: u64 = 0;

        use std::time::SystemTime;
        // siphasher
        let mut now = SystemTime::now();
        for _i in 1..total {
            sip_hasher.write(new_data[_i % new_data.len()]);
            a = a.wrapping_add(sip_hasher.finish());
        }
        let siphasher_then = now.elapsed().unwrap().as_millis();

        // xxh3_64 oneshot
        now = SystemTime::now();

        for _i in 1..total {
            a = a.wrapping_add(xxh3_64_with_seed(new_data[_i % new_data.len()], seed));
        }
        let xxh3_64_oneshot_then = now.elapsed().unwrap().as_millis();

        // xxh3_64 streaming
        now = SystemTime::now();
        for _i in 1..total {
            xxh3_hasher.write(new_data[_i % new_data.len()]);
            a = a.wrapping_add(xxh3_hasher.finish());
        }
        let xxh3_64_streaming_then = now.elapsed().unwrap().as_millis();

        // twox_hash oneshot
        now = SystemTime::now();
        for _i in 1..total {
            a = a.wrapping_add(twox_hash::XxHash3_64::oneshot_with_seed(
                seed,
                new_data[_i % new_data.len()],
            ));
        }
        let twox_oneshot_then = now.elapsed().unwrap().as_millis();

        println!(
            "| {} | {} | {}ms | {}ms | {}ms | {}ms | {} |",
            data_len,
            total,
            siphasher_then,
            xxh3_64_oneshot_then,
            xxh3_64_streaming_then,
            twox_oneshot_then,
            a
        );
    }
}
