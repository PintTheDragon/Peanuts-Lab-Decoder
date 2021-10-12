use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::str;
use std::iter::Map;
use std::ops::Index;
use std::slice::Iter;
use regex::{Match, Regex};
use substring::Substring;
use lazy_static::lazy_static;
use rust_embed::{RustEmbed, EmbeddedFile};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn main() {
    lazy_static! {
        static ref SCP_CHAR: Vec<char> = vec!['.', '*', '<', '>', '-'];
        static ref ALPHABET_CHAR: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        static ref SPLIT_REGEX: Regex = Regex::new(r"[.*<>-]+\W\[\d+x\d+x0\d+]\[\d+]").unwrap();
    }

    let mut map_idx: usize = 0;
    let alphabet_char_index: HashMap<char, usize> = ALPHABET_CHAR.iter().map(|&data| {map_idx += 1; (data, map_idx) }).collect();

    map_idx = 0;
    let scp_char_index: HashMap<char, usize> = SCP_CHAR.iter().map(|&data| {map_idx += 1; (data, map_idx) }).collect();

    let words_file: EmbeddedFile = Asset::get("words.txt").unwrap();
    let english_words = str::from_utf8(words_file.data.as_ref()).expect("An error occurred while reading the words list").split("\n").collect::<Vec<&str>>();

    let encoded_str = "<<<**.>..>>**. [33x11x020][5].*>**<**-.<..><>. [41x13x011][5]<>..<**-<> [28x9x08][1]><>...-<<*..>-**>*. [49x16x014][14]..*.*.**-*>... [26x6x02][2]*<..**...<>>.>.<*>*>*>><. [60x12x06][4]";
    let encoded_parts = SPLIT_REGEX.find_iter(encoded_str).collect::<Vec<Match>>();

    for to_decode in encoded_parts {
        let mut parts = to_decode.as_str().split(" ");
        let encoded1 = parts.next().expect("Received an invalidly formatted input");
        let info1 = parts.next().expect("Received an invalidly formatted input");

        let encoded = map_encoded(encoded1, &scp_char_index);
        let info = decode_info(info1);

        let min_second_value = get_min_second_value(encoded, info.first_value);

        //Slice the alphabet to only the ones that matter.
        let used_chars = ALPHABET_CHAR.iter().skip(info.lowest_value - 1).collect::<Vec<&char>>();

        let mut words: Vec<&str> = Vec::new();

        //Find all the words in our word list that match.

        let start_char = ALPHABET_CHAR[info.first_value - 1];
        let lowest_char = ALPHABET_CHAR[info.lowest_value - 1];

        for english_word in english_words.iter() {
            //Sure, this could be shortened, but it would be a massive if statement.

            let len = english_word.len();

            //Is the length correct?
            if len != info.size - 1 && len != info.size {
                continue;
            }
            //Is the first letter correct?
            if !english_word.starts_with(start_char) {
                continue;
            }
            //Is every character valid?
            if english_word.chars().any(|word_char: char| !used_chars.contains(&&word_char)) {
                continue;
            }
            //Is the second letter in the expected range, if it exists?
            //This is after the previous check and not before it because the list contains invalid characters.
            if len > 1 && alphabet_char_index[english_word.chars().nth(1).expect("An error occurred").borrow()] < min_second_value {
                continue;
            }
            //Is the lowest char used at least once?
            if !english_word.chars().any(|word_char: char| word_char == lowest_char) {
                continue;
            }
            //Does the word produce the same info?
            if !is_equivalent(info, english_word.chars().collect::<Vec<char>>(), len, alphabet_char_index.borrow()) {
                continue;
            }

            words.push(english_word);
        }

        println!("{}", words.join("\n") + "\n----------------");
    }
}

fn map_encoded(encoded: &str, scp_char_index: &HashMap<char, usize>) -> Vec<usize> {
    return encoded.chars().map(|char| scp_char_index[&char]).collect();
}

fn get_min_second_value(encoded: Vec<usize>, first_value: usize) -> usize {
    //Find the first encoded value directly after the end of the first character.

    let mut cur = first_value;

    for val in encoded {
        if cur < 1 {
            //Cur is less than one, this is the value we want.
            //This code does not account for cur being less than 0, as it should be impossible.
            return val;
        }
        cur -= val;
    }

    return 1;
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