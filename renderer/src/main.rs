use core::{Decoder, dump_gif};
use core::grammar::LogicalScreenDescriptor;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use eyre::Result;
use minifb::{Window, WindowOptions};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    gif_path: PathBuf,

    #[arg(long, short, default_value = "100")]
    frame_rate: u16,
}

fn main() -> Result<()> {
    let Args {
        gif_path,
        frame_rate,
    } = Args::parse();

    let data = dump_gif(gif_path.to_str().expect("Failed to find path"))?;
    let mut decoder = Decoder::new(data);
    let compressed_gif = decoder.parse()?;

    let LogicalScreenDescriptor {
        canvas_width,
        canvas_height,
        ..
    } = compressed_gif.logical_screen_descriptor;

    let mut window = Window::new(
        "GIF renderer",
        canvas_width as usize,
        canvas_height as usize,
        WindowOptions::default(),
    )?;

    window.set_target_fps(30);

    let frames = compressed_gif.decompress()?;

    while window.is_open() {
        for frame in &frames {
            let (image_descriptor, pixels) = frame;
            window.update_with_buffer(
                pixels,
                image_descriptor.image_width as usize,
                image_descriptor.image_height as usize,
            )?;
            sleep(Duration::from_millis(frame_rate as u64));
        }
    }

    Ok(())
}