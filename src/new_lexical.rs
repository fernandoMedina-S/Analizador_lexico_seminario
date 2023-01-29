#[path = "./utils/lexical.util.rs"] mod lexical_util;

pub struct LexicalAnalysis {
    original: String,
    tokens: Vec<String>,
    states: Vec<u32>,
    state: u32,
}

impl LexicalAnalysis {
    pub fn new(original: &str) -> Self {
        Self{
            original: String::from(original),
            tokens: Vec::new(),
            states: Vec::new(),
            state: 0,
        }
    }

    fn split_tokens(&mut self){
        let split = self.original.split_whitespace();
        for elem in split {
            self.tokens.push(String::from(elem));
        }
    }

    fn generate_state(&mut self, to_analize: &String){
        self.state = 0;
        for i in to_analize.chars(){
            if lexical_util::is_type(to_analize.clone()) {
                self.states.push(7);
                return;
            }
            if lexical_util::is_rel_type(to_analize.clone()) {
                self.states.push(9);
                return;
            }
            if lexical_util::is_logic_op(to_analize.clone()) != 0 {
                self.states.push(lexical_util::is_logic_op(to_analize.clone()));
                return;
            }
            if lexical_util::is_equal_op(to_analize.clone()) {
                self.states.push(13);
                return;
            }
            if lexical_util::is_condictional_op(to_analize.clone()) != 0 {
                self.states.push(lexical_util::is_condictional_op(to_analize.clone()));
                return;
            }
            if lexical_util::is_reserved_word(to_analize.clone()) != 0 {
                self.states.push(lexical_util::is_reserved_word(to_analize.clone()));
                return;
            }
            match self.state {
                0 => {
                    // Initial state
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 3;
                    } else if i == '+' || i == '-' {
                        self.state = 6;
                    } else if i == '*' || i == '/' {
                        self.state = 8;
                    } else if i == ';' {
                        self.state = 14;
                    } else if i == ',' {
                        self.state = 15;
                    } else if i == '(' {
                        self.state = 16;
                    } else if i == ')' {
                        self.state = 17;
                    } else if i == '{' {
                        self.state = 18;
                    } else if i == '}' {
                        self.state = 19;
                    } else if i == '=' {
                        self.state = 20;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                1 => {
                    // a - Z identifier
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 2;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                2 => {
                    //Numeric identifier
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 2;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                3 => {
                    //Float number initial
                    if i.is_digit(10) {
                        self.state = 3;
                    } else if i == '.' {
                        self.state = 4;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                4 => {
                    //Dot for Float number
                    if i.is_digit(10) {
                        self.state = 5;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                5 => {
                    //Additional digits for Float number
                    if i.is_digit(10) {
                        self.state = 5;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                _ => self.state = u32::MAX,
            }
        }
        self.states.push(self.state);
    }

    pub fn analize(&mut self) {
        self.split_tokens();
        let mut i: usize = 0;
        let tokens = self.tokens.clone();
        while i < self.tokens.len() {
            self.generate_state(&tokens[i]);
            i = i + 1;
        }
    }

    pub fn get_results(&self){
        let mut i: usize = 0;
        //println!("{:?}", self.states);
        for elem in &self.states {
            println!(
                "El token {} es de tipo {} \t{}",
                self.tokens[i],
                lexical_util::get_type(*elem),
                lexical_util::get_numerical_value(*elem)
            );
            i = i + 1;
        }
    }
}

impl Default for LexicalAnalysis{
    fn default() -> Self {
        Self {
            original: String::from(""),
            tokens: Vec::new(),
            states: Vec::new(),
            state: 0,
        }
    }
}