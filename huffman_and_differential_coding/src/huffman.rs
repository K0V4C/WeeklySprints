use std::io::{BufRead, BufReader};
use std::{collections::HashMap, fmt::Display, fs};
use std::fs::File;

#[derive(Debug)]
pub struct HuffmanNode {
    freq: usize,
    value: Option<i32>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(value: Option<i32>, freq: usize) -> Box<Self> {
        Box::new(HuffmanNode {
            freq,
            value,
            left: None,
            right: None,
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HuffTreeErr,
    Message(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HuffTreeErr => write!(f, "Failed to create Huffman tree"),
            Self::Message(msg) => msg.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Message(value.to_string())
    }
}

// LOADING FROM TXT FILE
fn load_table(path: &str) -> Result<HashMap<i32, usize>> {
    let mut freq_map: HashMap<i32, usize> = HashMap::new();

    let contents = fs::read_to_string(path).expect("Bad path!");

    let seperated = contents.trim().split('\n').collect::<Vec<&str>>();

    for x in seperated {
        let line = x.trim().split(':').collect::<Vec<&str>>();

        let (key, freq) = (
            line[0].trim().parse::<i32>().expect("Cant parse the number!"),
            line[1].trim().parse::<usize>().expect("Can't parse the number"),
        );

        freq_map.insert(key, freq);
    }

    Ok(freq_map)
}

// CREATING THE TREE
fn create_huffman_tree() -> Result<Box<HuffmanNode>> {
    let freq_map = load_table("data/freq_table.txt")?;

    let mut nodes: Vec<Box<HuffmanNode>> = freq_map
        .iter()
        .map(|(ch, freq)| HuffmanNode::new(Some(*ch), *freq))
        .collect();

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| b.freq.cmp(&a.freq));
        let a = nodes.pop().ok_or(Error::HuffTreeErr)?;
        let b = nodes.pop().ok_or(Error::HuffTreeErr)?;

        let mut node = HuffmanNode::new(None, a.freq + b.freq);
        node.left = Some(a);
        node.right = Some(b);
        nodes.push(node);
    }

    let root = nodes.pop().ok_or(Error::HuffTreeErr)?;

    Ok(root)
}

// BUIDLING FINAL TABLE
fn assign_huffman_codes(node: Box<HuffmanNode>, h: &mut HashMap<i32, String>, s: String) {
    if let Some(ch) = node.value {
        h.insert(ch, s);
    } else {
        if let Some(left) = node.left {
            assign_huffman_codes(left, h, s.clone() + "0");
        }
        if let Some(right) = node.right {
            assign_huffman_codes(right, h, s + "1");
        }
    }
}

pub fn generate_coding_table(huffman_code_table: &mut HashMap<i32, String>) -> Result<()> {
    let tree_root = create_huffman_tree()?;

    assign_huffman_codes(tree_root, huffman_code_table, "".to_string());

    // Only uncoment if u want to view encoding values
    // for x in &mut *huffman_code_table {
    //     println!("{} === {}", x.0, x.1);
    // }

    // Calculate density of encoding

    let prob_table = File::open("data/freq_table.txt").expect("Error opening file table");
    let reader = BufReader::new(prob_table);

    let mut lsr = 0.0;
    
    for line in reader.lines().flatten() {
        
        let split_line = line.trim().split(':').collect::<Vec<&str>>();

        let (value, _count, prob) = (
            split_line[0].trim().parse::<i32>().expect("Not a number!"),
            split_line[1].trim().parse::<i32>().expect("Not a number!"),
            split_line[2].trim().parse::<f64>().expect("Not a float")
        );

        let huffman_symbol_len = huffman_code_table.get(&value).expect("Huffman died :(").len();

        lsr += huffman_symbol_len as f64 * prob;

    }

    let density = 8 as f64 / lsr;

    println!("Cdoing denisity now is: {}", density);


    Ok(())
}
