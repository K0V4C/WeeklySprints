use std::{
    collections::HashMap,
    sync::mpsc,
    thread,
};

use differential_coding::{
    generate_coding_table, generate_frequency_table, reciever_thread, sender_thread,
};

fn main() {
    println!("This is a simple simulation for differential coding with static huffman!");

    let mut coding_table: HashMap<i32, String> = HashMap::new();
    let mut image_number: usize = 0;

    loop {
        println!(
            "
        Please choose an option:
        1) Create frequency table (required)
        2) Select picture
        3) Create Full huffman tree with step 1
        4) Start Simulation (create channel, ONLY RUN ONCE)
        5) Exit
        "
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let option: Option<i32> = match input.trim().parse() {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        if let Some(value) = option {
            match value {
                1 => generate_frequency_table().expect("Error creating table"),
                2 => {
                    println!(
                    "
                    \r\tPlease select which picture you want drawn:
                    \r\t0 -> Sekelton
                    \r\t1 -> Cartoony Hills
                    \r\t2 -> Tokyo
                    ");

                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let option: Option<usize> = match input.trim().parse() {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    };

                    image_number = option.expect("Not a number!");
                }
                3 => generate_coding_table(&mut coding_table).expect("Error compressing data"),
                4 => {
                    print!("Simulation Start!");
                    let (tx, rx) = mpsc::channel::<String>();
                    let freq_map = coding_table.clone();
                    let freq_map2 = coding_table.clone();

                    // SDL2 Thread for visualising the image
                    thread::spawn(move || {
                        reciever_thread(freq_map, rx);
                        println!("Reciever Thread o7!");
                    });

                    // Thread that sends data over
                    thread::spawn(move || {
                        sender_thread(freq_map2, tx, image_number).expect("Sender died");
                        println!("Sender Thread o7");
                    });


                }
                _ => break,
            }
        } else {
            println!("NaN Try again!");
        }
    }
}
