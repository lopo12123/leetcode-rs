struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let len = binary.len();
        let mut u8 = vec![49; len];

        // 0: 48; 1: 49
        let codes = binary.as_bytes();

        let mut first_zero = 0;
        loop {
            if first_zero == len || codes[first_zero] == 48 { break; }
            first_zero += 1;
        }

        let mut count_zero = 0u32;
        for i in first_zero..len {
            count_zero += if codes[i] == 48 { 1 } else { 0 };
        }

        if count_zero == 0 {
            binary
        } else {
            u8[first_zero + count_zero as usize - 1] = 48;
            String::from_utf8(u8).unwrap()
        }
    }
}