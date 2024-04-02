use std::fs::File;
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::mpsc::{Receiver, Sender},
};

use image::{io::Reader as ImageReader, GenericImageView};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub fn get_images() -> Result<Vec<String>, std::io::Error> {
    let mut image_names: Vec<String> = Vec::new();

    let dir_path = "data/images";

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;

        let file_name = entry.file_name().to_string_lossy().to_string();

        image_names.push(file_name);
    }

    Ok(image_names)
}

pub fn generate_frequency_table() -> Result<(), std::io::Error> {
    let image_names = get_images()?;

    let mut frequency_table: HashMap<i32, i32> = HashMap::new();

    for image_name in image_names {

        let freq_vek = get_coding_sequence(image_name.as_str())?;

        for el in freq_vek {
            *frequency_table.entry(el.0).or_insert(1) += 1;
            *frequency_table.entry(el.1).or_insert(1) += 1;
            *frequency_table.entry(el.2).or_insert(1) += 1;
        }
    }

    // Add other values to be 100% sure
    let mut set: HashSet<i32> = HashSet::new();

    for x in -255..=255 {
        let check = frequency_table.get(&x);
        if let None = check {
            set.insert(x);
        }
    }

    // Adding with lowest frequency
    for x in set {
        frequency_table.insert(x, 1);
    }

    let mut reverse_table: Vec<(i32, i32)> = vec![];
    for (key, value) in &frequency_table {
        reverse_table.push((*value, *key));
    }
    reverse_table.sort_by_key(|&(x, _)| x);

    // Create a file
    let mut data_file = File::create("data/freq_table.txt").expect("creation failed");

    println!("Creating frequency table!");

    for x in reverse_table {
        data_file
            .write(format!("{}:{}\n", x.1, x.0).as_bytes())
            .expect("Error while writing freq table!");
    }

    Ok(())
}

fn get_coding_sequence(image: &str) -> Result<Vec<(i32, i32, i32)>, std::io::Error> {
    let mut res_vek: Vec<(i32, i32, i32)> = vec![];

    let path = format!("data/images/{}", image);
    let img_result = ImageReader::open(path)?.decode();

    let img = match img_result {
        Ok(img) => img,
        Err(_err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error decoding the image",
            ))
        }
    };

    let (width, heigth) = img.dimensions();

    let mut last_values = (0, 0, 0);

    for y in 0..heigth {
        for x in 0..width {
            let (r, g, b) = (
                img.get_pixel(x, y)[0] as i32,
                img.get_pixel(x, y)[1] as i32,
                img.get_pixel(x, y)[2] as i32,
            );

            let (v1, v2, v3);

            if x == 0 && y == 0 {
                v1 = r;
                v2 = g;
                v3 = b;
            } else {
                v1 = r - last_values.0;
                v2 = g - last_values.1;
                v3 = b - last_values.2;
            }

            res_vek.push((v1, v2, v3));

            last_values = (r, g, b);
        }
    }

    return Ok(res_vek);
}

pub fn sender_thread(
    coding_table: HashMap<i32, String>,
    send: Sender<String>,
    img_num: usize,
) -> Result<(), std::io::Error> {
    let images = get_images().unwrap();

    let image = &images[img_num];

    let seq_vec = get_coding_sequence(&image)?;

    let mut test_send = File::create("data/test_send.txt").unwrap();

    for el in seq_vec {
        let (r, g, b) = el;

        let rs = coding_table
            .get(&r)
            .expect("cant find huffman code for r")
            .to_owned();

        let gs = coding_table
            .get(&g)
            .expect("cant find huffman code for g")
            .to_owned();
        let bs = coding_table
            .get(&b)
            .expect("cant find huffman code for b")
            .to_owned();

        test_send
            .write(
                format!(
                    "i32::  {}:{}:{}    strings::   {}:{}:{}\n",
                    r, g, b, rs, gs, bs
                )
                .as_bytes(),
            )
            .expect("write failed");

        send.send(rs).unwrap();
        send.send(gs).unwrap();
        send.send(bs).unwrap();
    }

    Ok(())
}

const WIDTH: u32 = 1920;
const HEIGTH: u32 = 1080;

pub fn reciever_thread(coding_table: HashMap<i32, String>, recieve: Receiver<String>) {
    let sdl_context = sdl2::init().expect("SDL2 failed init");
    let video_subsystem = sdl_context.video().expect("video died");

    let rev_coding_table: HashMap<&String, &i32> =
        coding_table.iter().map(|(k, v)| (v, k)).collect();

    let window = video_subsystem
        .window("OTR Domaci", WIDTH, HEIGTH)
        .position_centered()
        .build()
        .expect("Window died");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut events = sdl_context.event_pump().expect("Events died");
    let mut idx = 0;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut first_time = true;
    let mut last_values = (0, 0, 0);
    let (mut r, mut g, mut b);

    let mut test_recv = File::create("data/test_recv.txt").unwrap();

    'main_loop: loop {
        // Handle events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        // Rendering part
        // Recieve Huffman code
        let rs = recieve.recv();

        let rs = match rs {
            Ok(val) => val,
            Err(_) => return,
        };

        let gs = recieve.recv();

        let gs = match gs {
            Ok(val) => val,
            Err(_) => return,
        };

        let bs = recieve.recv();

        let bs = match bs {
            Ok(val) => val,
            Err(_) => return,
        };

        // Get value connected to huffman code
        let rm = **rev_coding_table.get(&rs).expect("Cant find reverse for r");
        let gm = **rev_coding_table.get(&gs).expect("Cant find reverse for g");
        let bm = **rev_coding_table.get(&bs).expect("Cant find reverse for b");

        // Produce true RGB value
        if first_time {
            first_time = !first_time;
            r = rm;
            g = gm;
            b = bm;
        } else {
            r = rm + last_values.0;
            g = gm + last_values.1;
            b = bm + last_values.2;
        }
        last_values = (r, g, b);

        test_recv
            .write(
                format!(
                    "i32::  {}:{}:{}    strings::   {}:{}:{}\n",
                    r, g, b, rs, gs, bs
                )
                .as_bytes(),
            )
            .expect("write failed");

        canvas.set_draw_color(Color::RGB(r as u8, g as u8, b as u8));

        let x = idx % WIDTH;
        let y = idx / WIDTH;

        canvas.draw_point((x as i32, y as i32)).unwrap();

        idx += 1;
        if idx == WIDTH * HEIGTH {
            idx = 0;
        }

        if idx % 1000 == 0 {
            canvas.present();
        }
    }
}
