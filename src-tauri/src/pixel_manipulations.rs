pub fn get_bits(byte: u8, amount: u8) -> u8 {
    if amount == 0 || amount > 8 {
        return 0;
    }

    if amount == 8 {
        return byte;
    }
    byte & ((1 << amount) - 1)
}

pub fn write_bits(byte: u8, bits: u8, amount: u8) -> u8 {
    if amount == 0 || amount > 8 {
        return byte;
    }

    if amount == 8 {
        return bits;
    }

    let mask = (1 << amount) - 1;
    (byte & !mask) | (bits & mask)
}

pub fn split_into_bits(bytes: &[u8], bits_per_chunk: u8) -> Vec<u8> {
    let bit_count = bytes.len() * 8;
    let bits_per_chunk = bits_per_chunk as usize;
    let chunks_count = (bit_count + bits_per_chunk - 1) / bits_per_chunk;
    let mut result = Vec::with_capacity(chunks_count);

    let mut current_byte = 0u8;
    let mut current_bits = 0;

    for byte in bytes.iter() {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            current_byte = (current_byte << 1) | bit;
            current_bits += 1;

            if current_bits == bits_per_chunk {
                result.push(current_byte);
                current_byte = 0;
                current_bits = 0;
            }
        }
    }

    if current_bits > 0 {
        current_byte <<= bits_per_chunk - current_bits;
        result.push(current_byte);
    }

    result
}

pub fn combine_bits(bytes: &[u8], bits_per_chunk: u8) -> Vec<u8> {
    let bits_per_chunk = bits_per_chunk as usize;
    let mut result = Vec::new();
    let mut current_byte = 0u8;
    let mut current_bits = 0;

    for &byte in bytes {
        for i in (0..bits_per_chunk).rev() {
            let bit = (byte >> i) & 1;

            current_byte = (current_byte << 1) | bit;
            current_bits += 1;

            if current_bits == 8 {
                result.push(current_byte);
                current_byte = 0;
                current_bits = 0;
            }
        }
    }

    if current_bits > 0 {
        current_byte <<= 8 - current_bits;
        result.push(current_byte);
    }

    result
}

pub fn convert_vec_to_single_bit(bits: Vec<u8>, bits_in_element: u8) -> Vec<u8> {
    let mut res = Vec::new();

    for bit in bits {
        for elem in 0..bits_in_element {
            res.push((bit >> (bits_in_element - 1 - elem)) & 1);
        }
    }

    res
}

#[test]
fn test_convert_vec_to_single_bit() {
    let message = vec![0b10101100, 0b01010101];
    assert_eq!(convert_vec_to_single_bit(message.clone(), 2), vec![0, 0, 0, 1]);
    assert_eq!(convert_vec_to_single_bit(message.clone(), 3), vec![1, 0, 0, 1, 0, 1]);
    assert_eq!(convert_vec_to_single_bit(message.clone(), 4), vec![1, 1, 0, 0, 0, 1, 0, 1]);
    assert_eq!(
        convert_vec_to_single_bit(message.clone(), 8),
        vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1]
    );
}
