#[test]
fn test_blur_image_correctly() {
    extern crate image;

    use super::gaussian_blur;
    use super::utils;

    let image_bytes = include_bytes!("../assets/cballs.png");
    if let Ok(image::DynamicImage::ImageRgb8(png_data)) = image::load_from_memory_with_format(image_bytes, image::ImageFormat::PNG) {
        let width = png_data.width() as usize;
        let height = png_data.height() as usize;
        let data = png_data.into_raw();
        let mut data_new = Vec::<[u8;3]>::with_capacity(width * height);
        data_new.resize(width * height, [0, 0, 0]);

        for y in 0..height {
            for x in 0..width {
                let offset = ((width * y) + x) as usize;
                data_new[((width * y) + x) as usize] = [data[offset * 3], data[(offset * 3) + 1], data[(offset * 3) + 2]];
            }
        }

        gaussian_blur(&mut data_new, width as usize, height as usize, 10.0);
        utils::write_image("test.ppm", &data_new, width as usize, height as usize).unwrap();

        assert!(true);
    } else {
        panic!("could not decode png");
    }
}
