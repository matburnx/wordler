use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::word_colors::WordColors;
use crate::word_colors::WORD_SIZE;
use crate::word_colors::NUMBER_OF_COLORS;

pub const VALID_WORDS_COUNT : u16 = 14855;
pub const FILE_NAME : &str = "valid_words.txt";

pub fn compare_words(guess_word: &[u8], given_word: &[u8]) -> WordColors {
  let mut given_word_colors: WordColors = WordColors::new();
  let mut is_colored : bool;
  for given_i in 0..given_word.len() {
    is_colored=false;
    for guess_i in 0..guess_word.len() {
      if given_word[given_i] == guess_word[guess_i] && given_i == guess_i {
        given_word_colors.greens.push((given_word[given_i] as char, given_i as u8));
        is_colored=true;
        break;
      } else if given_word[given_i] == guess_word[guess_i] {
        given_word_colors.yellows.push((given_word[given_i] as char, given_i as u8));
        is_colored=true;
        break;
      }
    }
    if !is_colored {
      given_word_colors.grays.push((given_word[given_i] as char, given_i as u8));
    }
  }
  assert!(given_word_colors.greens.len() + given_word_colors.yellows.len() + given_word_colors.grays.len() == WORD_SIZE as usize);
  return given_word_colors;
}

pub fn is_similar_word(given_colors: WordColors, other_word: &[u8]) -> bool {
  for green in &given_colors.greens {
    if other_word[green.1 as usize] as char != green.0 {
      return false;
    }
  }

  let mut is_in_word : bool;
  for yellow in &given_colors.yellows {
    is_in_word=false;
    for i in 0..other_word.len() {
      if i as u8 == yellow.1 && other_word[i] as char == yellow.0 {
        return false;
      }
      else if other_word[i] as char == yellow.0 {
        is_in_word=true;
      }
    }
    if !is_in_word {
      return false;
    }
  }

  for gray in &given_colors.grays {
    for letter in other_word {
      if *letter as char == gray.0 {
        return false;
      }
    }
  }

  return true;
}

pub fn count_similar_words(given_word: WordColors) -> u16 {
  let words_buffer = fs::read_to_string(FILE_NAME)
      .expect("Cannot open {FILE_NAME}");
  let words = words_buffer.lines();

  let mut word_count : u16 = 0;

  for word in words {
    if is_similar_word(given_word.clone(), word.as_bytes()) {
      word_count+=1;
    }
  }

  return word_count;
}

pub fn find_similar_words(given_word: WordColors) -> Vec::<String> {
  let words_buffer = fs::read_to_string(FILE_NAME)
      .expect("Cannot open file {FILE_NAME}");
  let words = words_buffer.lines();

  let mut possible_words = Vec::<String>::new();

  for word in words {
    if is_similar_word(given_word.clone(), word.as_bytes()) {
      possible_words.push(String::from(word));
    }
  }

  return possible_words;
}

fn compute_every_color_combination(given_word : &[u8]) -> Vec<WordColors> {
  let mut color_combinations : Vec<WordColors> = Vec::new();
  for a in 0..NUMBER_OF_COLORS {
    for b in 0..NUMBER_OF_COLORS {
      for c in 0..NUMBER_OF_COLORS {
        for d in 0..NUMBER_OF_COLORS {
          for e in 0..NUMBER_OF_COLORS {
            let mut color : WordColors = WordColors::new();
            color.add_color(a as u8,given_word[0] as char, 0);
            color.add_color(b as u8,given_word[1] as char, 1);
            color.add_color(c as u8,given_word[2] as char, 2);
            color.add_color(d as u8,given_word[3] as char, 3);
            color.add_color(e as u8,given_word[4] as char, 4);
            color_combinations.push(color);
          }
        }
      }
    }
  }
  return color_combinations;
}

pub fn compute_information(given_word : &[u8]) -> Vec<f64> {
  let mut weights : Vec<f64> = Vec::new();
  let combinations : Vec<WordColors> = compute_every_color_combination(given_word);

  for combi in combinations {
    let number_of_possible_words : u16 = count_similar_words(combi);
    let probability : f64 = number_of_possible_words as f64 / VALID_WORDS_COUNT as f64;
    let weight : f64 = (1./probability).log2() * probability;
    if !f64::is_nan(weight) {
      weights.push(weight);
    }
  }

  return weights;
}

pub fn total_information(given_word : &[u8]) -> f64 {
  let weights : Vec<f64> = compute_information(given_word);
  let mut total : f64 = 0.;

  for weight in weights {
    total+=weight;
  }

  return total;
}

pub fn information_of_every_word() -> Vec<f64> {
  let words_buffer = fs::read_to_string(FILE_NAME)
      .expect("Cannot open {FILE_NAME}");
  let words = words_buffer.lines();

  let path = Path::new("word_weights.txt");
  let display = path.display();

  let mut file = File::create(&path)
    .expect(format!("Failed to create {}", display).as_str());

  let mut weights : Vec<f64> = Vec::new();

  for word in words {
    let weight : f64 = total_information(word.as_bytes());
    weights.push(weight);
    file.write_all(format!("{:.3}\n", weight).as_bytes())
      .expect(format!("couldn't write to {}", display).as_str());
    println!("{weight}");
  }

  return weights;
}
