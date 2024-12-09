use image::{ DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba };

pub fn hide(
    container: DynamicImage,
    message: &[u8],
    bits_per_channel: u8
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (w, h) = container.dimensions();

    let mut res = ImageBuffer::new(w, h);

    let bits = split_into_bits(message, bits_per_channel);
    let mut iter = bits.iter().peekable();

    for (x, y, mut pixel) in container.pixels() {
        let channels = pixel.channels_mut();

        if iter.peek().is_some() {
            for channel in &mut channels[..3] {
                if let Some(bits) = iter.next() {
                    *channel = write_bits(*channel, *bits, bits_per_channel);
                }
            }
        }

        res.put_pixel(x, y, pixel);
    }

    res
}

pub fn extract(container: DynamicImage, message_size: usize, bits_per_channel: u8) -> Vec<u8> {
    const CHANNELS_AMOUNT: usize = 3;
    let take_amount = (message_size * 8) / (CHANNELS_AMOUNT * (bits_per_channel as usize)) + 1;

    let iter = container
        .pixels()
        .take(take_amount)
        .flat_map(|(_, _, pixel)| {
            let channels = pixel.channels();

            let res = (0..3)
                .map(move |i| get_bits(channels[i], bits_per_channel))
                .collect::<Vec<u8>>();

            res
        });

    let bits: Vec<u8> = iter.collect();

    combine_bits(&bits, bits_per_channel)[..message_size].to_vec()
}
pub fn image_matrix(image: DynamicImage, file_name: &str) {
    let mut container = ImageBuffer::new(image.width(), image.height());

    for (x, y, pixel) in image.pixels() {
        container.put_pixel(x, y, map_pixel(pixel));
    }

    container.save(file_name).unwrap();
}

fn map_pixel(mut pixel: Rgba<u8>) -> Rgba<u8> {
    let channels = pixel.channels_mut();

    channels[0] = if (channels[0] & 1) == 1 { 255 } else { 0 };
    channels[1] = if (channels[1] & 1) == 1 { 255 } else { 0 };
    channels[2] = if (channels[2] & 1) == 1 { 255 } else { 0 };

    pixel
}

fn get_bits(byte: u8, amount: u8) -> u8 {
    if amount == 0 || amount > 8 {
        return 0;
    }
    byte & ((1 << amount) - 1)
}

fn write_bits(byte: u8, bits: u8, amount: u8) -> u8 {
    if amount == 0 || amount > 8 {
        return byte;
    }
    let mask = (1 << amount) - 1;
    (byte & !mask) | (bits & mask)
}

fn split_into_bits(bytes: &[u8], bits_per_chunk: u8) -> Vec<u8> {
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

fn combine_bits(bytes: &[u8], bits_per_chunk: u8) -> Vec<u8> {
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
