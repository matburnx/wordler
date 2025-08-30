  pub const WORD_SIZE: u8 = 5;
  pub const NUMBER_OF_COLORS: u8 = 3;
  
  pub struct WordColors{
    pub greens: Vec<(char, u8)>,
    pub yellows: Vec<(char, u8)>,
    pub grays: Vec<(char, u8)>,
  }
  
  impl WordColors {
    pub fn new() -> Self {
      Self {
        greens: Vec::new(),
        yellows: Vec::new(),
        grays: Vec::new(),
      }
    }

    pub fn clone(&self) -> WordColors {
      let copy : WordColors = WordColors {
        greens: (*self.greens.clone()).to_vec(),
        yellows: (*self.yellows.clone()).to_vec(),
        grays: (*self.grays.clone()).to_vec(),
      };
      return copy;
    }

    pub fn add_color(&mut self, color_index : u8, character : char, index : u8) {
      match color_index{
        0=> (*self).grays.push((character,index)),
        1=> (*self).yellows.push((character,index)),
        2=> (*self).greens.push((character,index)),
        _=> panic!("Index {} out of boud", color_index),
      }
    }
  }
