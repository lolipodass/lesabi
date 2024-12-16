use image::{ DynamicImage, GenericImageView, ImageBuffer, Pixel, Pixels, Rgba };

use crate::pixel_manipulations::{
    combine_bits,
    convert_vec_to_single_bit,
    get_bits,
    split_into_bits,
    write_bits,
};

pub fn hide(
    container: DynamicImage,
    message: &[u8],
    bits_per_channel: u8
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    if bits_per_channel == 0 || bits_per_channel > 8 {
        return Err("Invalid bits per channel".to_string());
    }

    if message.is_empty() {
        return Err("Message is empty".to_string());
    }

    let (w, h) = container.dimensions();
    let max_message_size = ((w * h * 3 * (bits_per_channel as u32)) / 8) as usize;

    if message.len() > max_message_size {
        return Err("Message too large for container image".to_string());
    }
    let (w, h) = container.dimensions();

    let mut res = ImageBuffer::new(w, h);

    let message_size = (message.len() as u64).to_be_bytes();

    let message_size = split_into_bits(&message_size, bits_per_channel);

    let bits = split_into_bits(message, bits_per_channel);
    let mut iter = message_size.iter().chain(bits.iter()).peekable();

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

    //add hidden check pixel
    let pixel = Rgba([1, 1, 1, 0]);

    res.put_pixel(w - 1, h - 1, pixel);
    Ok(res)
}

pub fn extract(container: DynamicImage, bits_per_channel: u8) -> Result<Vec<u8>, String> {
    const CHANNELS_AMOUNT: u64 = 3;

    let (w, h) = container.dimensions();

    if w < 10 || h < 10 {
        return Err("Image too small".to_string());
    }

    let pixel = container.get_pixel(w - 1, h - 1);

    if pixel[0] != 1 || pixel[1] != 1 || pixel[2] != 1 {
        return Err("No hidden message marker found".to_string());
    }

    if bits_per_channel == 0 || bits_per_channel > 8 {
        return Err("Invalid bits per channel".to_string());
    }

    let mut pixels = container.pixels();

    let pixels_for_length = calculate_required_pixels(8, bits_per_channel, CHANNELS_AMOUNT);

    let length_bits = read_bits_from_iter(&mut pixels, pixels_for_length, bits_per_channel);

    let length_bit_vector = convert_vec_to_single_bit(length_bits, bits_per_channel);

    let actual_message_length = u64::from_be_bytes(
        combine_bits(&length_bit_vector[..64], 1)
            .try_into()
            .unwrap()
    );

    if actual_message_length == 0 {
        return Ok(Vec::new());
    }
    let mut extracted_message_bits = length_bit_vector[64..].to_vec();

    let message_size = calculate_required_pixels(
        actual_message_length,
        bits_per_channel,
        CHANNELS_AMOUNT
    );

    extracted_message_bits.extend(
        convert_vec_to_single_bit(
            read_bits_from_iter(&mut pixels, message_size, bits_per_channel),
            bits_per_channel
        )
    );

    Ok(combine_bits(&extracted_message_bits, 1)[..actual_message_length as usize].to_vec())
}

fn take_bits_from_pixel(pixel: Rgba<u8>, bits_in_channel: u8) -> Vec<u8> {
    let channels = pixel.channels();

    (0..3).map(move |i| get_bits(channels[i], bits_in_channel)).collect()
}

fn read_bits_from_iter(
    iter: &mut Pixels<'_, DynamicImage>,
    read_amount: u64,
    bits_in_channel: u8
) -> Vec<u8> {
    let mut res = Vec::new();

    for _ in 0..read_amount {
        let (_, _, pixel) = iter.next().unwrap();
        let bits = take_bits_from_pixel(pixel, bits_in_channel);
        res.extend_from_slice(&bits);
    }

    res
}

fn calculate_required_pixels(size: u64, bits_per_channel: u8, channel_amount: u64) -> u64 {
    const BYTE: u64 = 8;
    let bits_per_pixel = bits_per_channel as u64;
    let bits_required = size * BYTE;
    let pixels_required = bits_required / (bits_per_pixel * channel_amount);
    if bits_required % (bits_per_pixel * channel_amount) > 0 {
        pixels_required + 1
    } else {
        pixels_required
    }
}

#[test]
fn test_stego() {
    let mut img = ImageBuffer::new(250, 250);
    for x in 0..250 {
        for y in 0..250 {
            img.put_pixel(x, y, Rgba([x as u8, y as u8, 0, 0]));
        }
    }
    let message = b"Hello world";
    for bit in 1..=8 {
        println!("\n\n\nbit {}", bit);

        let hidden = hide(img.clone().try_into().unwrap(), message, bit).unwrap();
        let res = extract(hidden.try_into().unwrap(), bit).unwrap();
        // assert_eq!(res, message);
        println!("equal? {}", res == message);
    }
    panic!()
}
