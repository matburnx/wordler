use std::io;

use crate::wordle::information_of_every_word;

pub mod wordle;
pub mod word_colors;

fn main() {
//  let frequencies: [f32; 26] = [8.2, 1.5, 2.8, 4.3, 12.7, 2.2, 2.0, 6.1, 7.0, 0.15, 0.77, 4.0, 2.4, 6.7, 7.5, 1.9, 0.095, 6.0, 6.3, 9.1, 2.8, 0.98, 2.4, 0.15, 2.0, 0.074];
/*
  let guess_word : String = String::from("weary");
  let guess_word_bytes = guess_word.as_bytes();
  assert!(guess_word_bytes.len() == WORD_SIZE as usize);


  println!("Guess the word");
  let mut input_word : String = String::new();
  io::stdin()
    .read_line(&mut input_word)
    .expect("Failed to read the word");

  let given_word = input_word.trim().to_lowercase();
  let given_word_bytes = given_word.as_bytes();
  assert!(given_word_bytes.len() == WORD_SIZE);

  let given_word_colors: WordColors = compare_words(guess_word_bytes,given_word_bytes);
*/
  let weights : Vec<f64> = information_of_every_word();
}

/*
A 	8.2 	
B 	1.5 	
C 	2.8 	
D 	4.3
E 	12.7 	
F 	2.2 	
G 	2.0 	
H 	6.1 	
I 	7.0 	
J 	0.15 	
K 	0.77 	
L 	4.0 	
M 	2.4 	
N 	6.7 	
O 	7.5 	
P 	1.9 	
Q 	0.095	
R 	6.0 	
S 	6.3 	
T 	9.1 	
U 	2.8 
V 	0.98 	
W 	2.4 	
X 	0.15 	
Y 	2.0 	
Z 	0.074	
*/
