use show_image::{create_window, ImageInfo, ImageView};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixel_data: &[u8; 100] = &[0; 100];
    let image = ImageView::new(ImageInfo::rgb8(10, 10), pixel_data);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    Ok(())
}
