extern crate image;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

use image::ImageBuffer;
use image::Rgb;
use clap::clap_app;

enum Action {
	WRITE,
	READ,
}

pub struct Config {
	image_path: String,
	silentmark_path: String,
	action: Action,
}

impl Config {
	pub fn new() -> Result<Config, &'static str> {

		let matches = clap_app!(silenmark =>
			(@setting SubcommandRequiredElseHelp)
			(version: "0.1.0")
			(author: "James Taylor <jamestaylor3@protonmail.com>")
			(about: "Use stenography on images from the command line")
			(@subcommand write =>
				(about: "Write a message to an image")
				(@arg message: -m +takes_value +required "path of image to read message")
				(@arg image: -i +takes_value +required "path of image to write message")
			)
			(@subcommand read =>
				(about: "Read a message from an image")
				(@arg image: -i +takes_value +required "path of image to read message")
				(@arg message: -o +takes_value "path of file to write message. default msg.txt")
			)
		).get_matches();

		let action: Action;

		if let Some(_) = matches.subcommand_matches("write") {
			action = Action::WRITE;
		} else if let Some(_) = matches.subcommand_matches("read") {
			action = Action::READ;
		} else {
			return Err("Not a valid subcommand. This should be unreachable!");
		}

		let silentmark_path = matches.value_of("message").unwrap_or("msg.txt").to_string();
		let image_path = matches.value_of("image").unwrap().to_string();

		Ok(Config { image_path, silentmark_path, action })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let mut img_buf = image::open(config.image_path)?.to_rgb();

	match config.action {
		Action::WRITE => write_silentmark(&mut img_buf, &config.silentmark_path),
		Action::READ => read_silentmark(&img_buf, &config.silentmark_path),
	}
}

// rename
const FIRST6: u8 = 0xfc;
const LAST2: u8 = 0x3;

fn write_silentmark(img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, silentmark_path: &str) -> Result<(), Box<dyn Error>> {
	let mut silentmark_file = File::open(silentmark_path)?;

	let mut buffer = Vec::new();
	silentmark_file.read_to_end(&mut buffer)?;

	let mut diced_silentmark = Vec::with_capacity(4 * buffer.len());

	for val in buffer.iter() {
		let mut x = *val;
		for _ in 0..4 {
			diced_silentmark.push(x & LAST2);
			x = x >> 2;
		}
	}

	for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
		let loc = 3 * (x + y) as usize;
		let len = diced_silentmark.len() as usize;
		let image::Rgb(data) = *pixel;
		*pixel = image::Rgb([
			data[0] & FIRST6 ^ diced_silentmark.get(loc % len).unwrap(),
			data[1] & FIRST6 ^ diced_silentmark.get((loc + 1) % len).unwrap(),
			data[2] & FIRST6 ^ diced_silentmark.get((loc + 2) % len).unwrap(),
		]);
	}

	img_buf.save(format!("silentmarked-out.ppm"))?;

	Ok(())
}

fn read_silentmark(img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>, silentmark_path: &str) -> Result<(), Box<dyn Error>> {
	let dim = img_buf.dimensions();
	let mut silentmark_dice_buffer = Vec::with_capacity(3 * (dim.0 + dim.1) as usize);

	for pixel in img_buf.pixels() {
		let image::Rgb(data) = *pixel;
		for i in 0..3 {
			silentmark_dice_buffer.push(data[i] & LAST2);
		}
	}

	let mut silentmark_dice_buffer = silentmark_dice_buffer.iter();
	let mut silentmark_buffer = vec![0; silentmark_dice_buffer.len() / 4];

	for item in silentmark_buffer.iter_mut() {
		for i in 0..4 {
			*item += silentmark_dice_buffer.next().unwrap() << 2 * (i);
		}
	}

	let mut outfile = File::create(silentmark_path)?;

	outfile.write_all(&silentmark_buffer)?;

	Ok(())
}