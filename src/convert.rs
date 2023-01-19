//! Module that contains the conversion functions.

/// Represent a YUV420 Pixel.
struct YUVPixel {
    y: u8,
    u: u8,
    v: u8,
}

/// Convert an RGB pixel to YUV420.
/// # Arguments
/// * `r` - Red component of the pixel.
/// * `g` - Green component of the pixel.
/// * `b` - Blue component of the pixel.
/// # Returns
/// A YUVPixel.
/// # Example
/// ```
/// use convert::YUVPixel;
/// let pixel = convert_rgb_to_yuv420_pixel(255, 0, 0);
/// assert_eq!(pixel.y, 255);
/// assert_eq!(pixel.u, 128);
/// assert_eq!(pixel.v, 128);
/// ```
fn convert_rgb_to_yuv420_pixel(r: f32, g: f32, b: f32) -> YUVPixel {
    YUVPixel {
        y: ((77_f32 * r + 150_f32 * g + 29_f32 * b + 128.0) as i32 >> 8).clamp(0, 255)
            as u8,
        u: (((-43_f32 * r - 84_f32 * g + 127_f32 * b + 128.0) as i32 >> 8) + 128)
            .clamp(0, 255) as u8,
        v: (((127_f32 * r - 106_f32 * g - 21_f32 * b + 128.0) as i32 >> 8) + 128)
            .clamp(0, 255) as u8,
    }
}

/// Convert an RGB buffer array to YUV420.
///
/// # Arguments
/// * `width` - The width of the image.
/// * `height` - The height of the image.
/// * `rgb` - The RGB buffer array.
/// * `bytes_per_pixel` - The number of bytes per pixel.
///
/// # Returns
/// A YUV420 buffer array.
///
/// # Example
/// ```
/// use convert::convert_rgb_to_yuv420;
/// let rgb = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/// let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
/// assert_eq!(yuv, vec![0, 0, 0, 0, 0, 0]);
/// ```
pub fn convert_rgb_to_yuv420(
    width: u32,
    height: u32,
    rgb: &[u8],
    bytes_per_pixel: usize,
) -> Vec<u8> {
    let frame_size: usize = (width * height) as usize;
    let chroma_size: usize = frame_size / 4;
    let mut y_index: usize = 0;
    let mut uv_index: usize = frame_size;
    let mut yuv: Vec<u8> = vec![0; (width * height * 3 / 2) as usize];
    let mut index: usize = 0;
    for j in 0..height {
        for _ in 0..width {
            let r: f32 = rgb[index * bytes_per_pixel] as f32;
            let g: f32 = rgb[index * bytes_per_pixel + 1] as f32;
            let b: f32 = rgb[index * bytes_per_pixel + 2] as f32;
            index += 1;
            let yuv_pixel = convert_rgb_to_yuv420_pixel(r, g, b);
            yuv[y_index] = yuv_pixel.y;
            y_index += 1;
            if j % 2 == 0 && index % 2 == 0 {
                yuv[uv_index] = yuv_pixel.u;
                yuv[uv_index + chroma_size] = yuv_pixel.v;
                uv_index += 1;
            }
        }
    }
    yuv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yuv_pixel_1() {
        let yuv_pixel = convert_rgb_to_yuv420_pixel(255.0, 0.0, 0.0);
        assert_eq!(yuv_pixel.y, 77);
        assert_eq!(yuv_pixel.u, 85);
        assert_eq!(yuv_pixel.v, 255);
    }

    #[test]
    fn yuv_pixel_2() {
        let yuv_pixel = convert_rgb_to_yuv420_pixel(0.0, 255.0, 0.0);
        assert_eq!(yuv_pixel.y, 149);
        assert_eq!(yuv_pixel.u, 44);
        assert_eq!(yuv_pixel.v, 22);
    }

    #[test]
    fn yuv_pixel_3() {
        let yuv_pixel = convert_rgb_to_yuv420_pixel(0.0, 0.0, 255.0);
        assert_eq!(yuv_pixel.y, 29);
        assert_eq!(yuv_pixel.u, 255);
        assert_eq!(yuv_pixel.v, 107);
    }

    #[test]
    fn yuv_pixel_4() {
        let yuv_pixel = convert_rgb_to_yuv420_pixel(255.0, 255.0, 255.0);
        assert_eq!(yuv_pixel.y, 255);
        assert_eq!(yuv_pixel.u, 128);
        assert_eq!(yuv_pixel.v, 128);
    }

    #[test]
    fn rgb_to_yuv_1() {
        let rgb = vec![255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![255, 0, 0, 0, 128, 128]);
    }

    #[test]
    fn rgb_to_yuv_2() {
        let rgb = vec![255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![255, 77, 149, 29, 85, 255]);
    }

    #[test]
    fn rgb_to_yuv_3() {
        let rgb = vec![255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![77, 77, 77, 77, 85, 255]);
    }

    #[test]
    fn rgb_to_yuv_4() {
        let rgb = vec![0, 255, 0, 255, 0, 0, 255, 0, 0, 0, 255, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![149, 77, 77, 149, 85, 255]);
    }

    #[test]
    fn rgb_to_yuv_5() {
        let rgb = vec![0, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 255];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![29, 255, 255, 29, 128, 128]);
    }

    #[test]
    fn rgb_to_yuv_6() {
        let rgb = vec![255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 0, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![255, 255, 0, 77, 128, 128]);
    }

    #[test]
    fn rgb_to_yuv_7() {
        let rgb = vec![42, 42, 42, 42, 42, 0, 42, 42, 0, 42, 42, 42];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![42, 37, 37, 42, 107, 131]);
    }

    #[test]
    fn rgb_to_yuv_8() {
        let rgb = vec![42, 42, 42, 0, 42, 0, 0, 42, 0, 0, 42, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 3);
        assert_eq!(yuv, vec![42, 25, 25, 25, 114, 111]);
    }

    #[test]
    fn rgb_to_yuv_4_bytes() {
        let rgb = vec![42, 42, 42, 0, 0, 42, 0, 0, 0, 42, 0, 0, 0, 42, 0, 0];
        let yuv = convert_rgb_to_yuv420(2, 2, &rgb, 4);
        assert_eq!(yuv, vec![42, 25, 25, 25, 114, 111]);
    }
}
