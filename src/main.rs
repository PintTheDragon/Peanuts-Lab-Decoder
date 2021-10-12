use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::Map;
use std::ops::Index;
use std::slice::Iter;
use regex::{Match, Regex};
use substring::Substring;
use lazy_static::lazy_static;

fn main() {
    lazy_static! {
        static ref SCP_CHAR: Vec<char> = vec!['.', '*', '<', '>', '-'];
        static ref ALPHABET_CHAR: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        static ref SPLIT_REGEX: Regex = Regex::new(r"[.*<>-]+\W\[\d+x\d+x0\d+]\[\d+]").unwrap();
    }

    let mut map_idx: usize = 0;
    let alphabet_char_index: HashMap<char, usize> = ALPHABET_CHAR.iter().map(|&data| {map_idx += 1; (data, map_idx) }).collect();

    let words_file = fs::read_to_string("./src/words.txt").expect("An error occurred while reading the words list");
    let english_words = words_file.split("\n").collect::<Vec<&str>>();

    let encoded_str = "<<<**.>..>>**. [33x11x020][5].*>**<**-.<..><>. [41x13x011][5]<>..<**-<> [28x9x08][1]><>...-<<*..>-**>*. [49x16x014][14]..*.*.**-*>... [26x6x02][2]*<..**...<>>.>.<*>*>*>><. [60x12x06][4]";
    let encoded_parts = SPLIT_REGEX.find_iter(encoded_str).collect::<Vec<Match>>();

    for to_decode in encoded_parts {
        let mut parts = to_decode.as_str().split(" ");
        let encoded1 = parts.next().expect("Received an invalidly formatted input");
        let info1 = parts.next().expect("Received an invalidly formatted input");

        let info = decode_info(info1);

        //Slice the alphabet to only the ones that matter.
        let used_chars = ALPHABET_CHAR.iter().skip(info.lowest_value - 1).collect::<Vec<&char>>();

        let mut words: Vec<&str> = Vec::new();

        //Find all the words in our word list that match.

        let start_char = ALPHABET_CHAR[info.first_value - 1];

        for english_word in english_words.iter() {
            //Sure, this could be shortened, but it would be a massive if statement.
            if english_word.len() != info.size - 1 && english_word.len() != info.size {
                continue;
            }
            if !english_word.starts_with(start_char) {
                continue;
            }
            if english_word.chars().any(|word_char: char| !used_chars.contains(&&word_char)) {
                continue;
            }
            if !is_equivalent(info, english_word.chars().collect::<Vec<char>>(), english_word.len(), alphabet_char_index.borrow()) {
                continue;
            }

            words.push(english_word);
        }

        println!("{}", words.join("\n") + "\n----------------");
    }
}

fn is_equivalent(info: Info, chars: Vec<char>, size: usize, alphabet_char_index: &HashMap<char, usize>) -> bool {
    let mut sum: usize = 0;
    for c in chars {
        sum += alphabet_char_index[&c];
    }

    return info.word_value == sum && info.average == sum / size;
}

fn decode_info(info: &str) -> Info {
    let mut parts = info.split("]");

    let mut part_a = substring_no_end(parts.next().expect("Received an invalidly formatted input"), 1).split("x");
    let part_b = substring_no_end(parts.next().expect("Received an invalidly formatted input"), 1);

    let word_value = part_a.next().expect("Received an invalidly formatted input").parse::<usize>().expect("Received an invalidly formatted input");
    let average = part_a.next().expect("Received an invalidly formatted input").parse::<usize>().expect("Received an invalidly formatted input");

    return Info {
        word_value,
        average,
        first_value: part_a.next().expect("Received an invalidly formatted input").parse::<usize>().expect("Received an invalidly formatted input"),
        lowest_value: part_b.parse::<usize>().expect("Received an invalidly formatted input"),
        size: word_value/average
    };
}

#[derive(Clone, Copy)]
struct Info {
    /// The summed value of each letter in a word.
    pub word_value: usize,
    /// The average value of every character (word value / size).
    pub average: usize,
    /// The value of the first letter.
    pub first_value: usize,
    /// The lowest value in this word.
    pub lowest_value: usize,
    /// Gets the size or size + 1.
    pub size: usize
}

fn substring_no_end(string: &str, start: usize) -> &str {
    return string.substring(start, string.len());
}