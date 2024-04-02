mod coding;
mod huffman;

pub use coding::{generate_frequency_table, get_images, reciever_thread, sender_thread};

pub use huffman::generate_coding_table;
