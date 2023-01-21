pub struct LexicalAnalysis {
    pub original: String,
    pub state: u32,
    pub l_type: String,
    pub curr_char: char,
    pub index: u32,
}

impl LexicalAnalysis {
    pub fn new(original: &str) -> Self {
        Self { original: String::from(original), state: 0, l_type: String::from(""), curr_char: ' ', index: 0 }
    }

    pub fn analize(&mut self) {
        for i in self.original.chars()  {
            match self.state {
                0 => if i.is_alphabetic() {
                    self.state = 1;
                } else if i.is_digit(10){
                    self.state = 3;
                } else {
                    self.state = u32::MAX;
                },
                1 => if i.is_alphabetic() {
                    self.state = 1;
                } else if i.is_digit(10){
                    self.state = 2;
                }else{
                    self.state = u32::MAX;
                },
                2 => if i.is_alphabetic() {
                    self.state = 1;
                } else if i.is_digit(10) {
                    self.state = 2;
                }else {
                    self.state = u32::MAX;
                },
                3 => if i.is_digit(10) {
                    self.state = 3;
                }else if i == '.' {
                    self.state = 4;
                }else{
                    self.state = u32::MAX;
                },
                4 => if i.is_digit(10) {
                    self.state = 5;
                }else{
                    self.state = u32::MAX;
                },
                5 => if i.is_digit(10) {
                    self.state = 5;
                }else{
                    self.state = u32::MAX;
                },
                _ => self.state = u32::MAX,
            }
        }
    }

    pub fn get_type(&mut self) -> String {
        match self.state {
            0 => return String::from("Cadena vacía"),
            1 | 2 => return String::from("Identificador"),
            5  => return String::from("Número Real"),
            u32::MAX => String::from("Error de sintaxis"),
            _ => String::from("Error de sintaxis"),
        }
    }
}

impl Default for LexicalAnalysis {
    fn default() -> Self {
        Self { original: String::from(""), state: 0, l_type: String::from(""), curr_char: ' ', index: 0 }
    }
}