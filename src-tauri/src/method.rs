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
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
    res
}

pub fn extract(container: DynamicImage, bits_per_channel: u8) -> Vec<u8> {
    const CHANNELS_AMOUNT: u64 = 3;

    let (w, h) = container.dimensions();

    if w < 10 || h < 10 {
        return Vec::new();
    }

    let pixel = container.get_pixel(w - 1, h - 1);

    if pixel[0] != 1 || pixel[1] != 1 || pixel[2] != 1 {
        return Vec::new();
    }

    let mut pixels = container.pixels();

    let amount_pixel_to_len = calculate_required_pixels(8, bits_per_channel, CHANNELS_AMOUNT);

    let message_size = read_bits_from_iter(&mut pixels, amount_pixel_to_len, bits_per_channel);

    let message_len = u64::from_be_bytes(
        combine_bits(&message_size[..64], bits_per_channel)
            .try_into()
            .unwrap()
    );
    let mut res = message_size[64..].to_vec();

    let message_size = calculate_required_pixels(message_len, bits_per_channel, CHANNELS_AMOUNT);

    res.extend(read_bits_from_iter(&mut pixels, message_size, bits_per_channel));

    combine_bits(&res, bits_per_channel)[..message_len as usize].to_vec()
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

        }
    }

    if current_bits > 0 {
        current_byte <<= 8 - current_bits;
        result.push(current_byte);
    }

    result
}
