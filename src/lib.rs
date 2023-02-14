use xcb::{x::{self}};
use xcb::x::{GetImageReply,Setup,ImageOrder};
use image::{RgbaImage, Rgba};

pub fn screenshot() -> xcb::Result<RgbaImage> {
    let (conn, screen_num) = xcb::Connection::connect(None)?;
    
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let width = screen.width_in_pixels();
    let height = screen.height_in_pixels();

    let cookie = conn.send_request(
        &x::GetImage{
            format: x::ImageFormat:: ZPixmap,
            drawable: x::Drawable::Window(screen.root()),
            x: 0,
            y: 0,
            width: width,
            height: height,
            plane_mask: u32::MAX,

        }
    );

    let reply = conn
        .wait_for_reply(cookie)?;
    
    Ok(img_from_reply(reply, setup, height as u32, width as u32))
}

// Assumes 24bit color like my machine
// TODO: Test on other machines running x11 (good luck lol)
fn img_from_reply(reply: GetImageReply, setup: &Setup, height: u32, width: u32) -> RgbaImage {
    let data = reply.data();
    let order = setup.image_byte_order();

    let pixels: Vec<Rgba<u8>> = data.chunks(4).map(|x| chunktopixel(x, order)).collect();
    let mut image = RgbaImage::new(width, height);
    for pixel in pixels.iter().enumerate() {
        let x = pixel.0 as u32 % width;
        let y = pixel.0 as u32 / width;
        image.put_pixel(x, y, pixel.1.clone())
    }
    image
}

fn chunktopixel(chunk: &[u8], order: ImageOrder) -> Rgba<u8> {
    if chunk.len() != 4 {
        panic!("Data length is not divisible by 4")
    }
    let mut pixel = chunk.iter();
    let red: u8;
    let blue: u8;
    let green: u8;
    if order == ImageOrder::LsbFirst {
        blue = *pixel.next().unwrap();
        green = *pixel.next().unwrap();
        red = *pixel.next().unwrap();
    }
    else {
        red = *pixel.next().unwrap();
        green = *pixel.next().unwrap();
        blue = *pixel.next().unwrap();
    }
    let alpha = *pixel.next().unwrap();
    let pixel = Rgba::<u8>{0:[red,green,blue,alpha]};
    pixel
}