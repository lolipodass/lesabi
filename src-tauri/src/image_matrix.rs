use image::{ DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba };

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
