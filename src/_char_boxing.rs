const BOX_PADDING: i32 = 10;

fn gen_img(code: &str) -> Image<Rgba<u8>> {
    // ...
    // let scale = Scale::uniform(scale);

    // Get character box
    let char_box = FONT
        .glyph(c)
        .scaled(scale)
        .positioned(point(x as f32, y as f32))
        .pixel_bounding_box()
        .expect(&format!("Failed to get bounding box for character: {}", c));

    // Calculate box for character
    let box_x = char_box.min.x as i32 - BOX_PADDING;
    let box_y = char_box.max.y as i32 + char_box.height() as i32 / 2 - BOX_PADDING;
    let box_width = (char_box.width() + BOX_PADDING * 2) as u32;
    let box_height = (char_box.height() + BOX_PADDING * 2) as u32;
    let rect = Rect::at(box_x, box_y).of_size(box_width, box_height);

    // // Random bright color
    // let color = Rgba([
    // ...
}
