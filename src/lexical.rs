pub struct LexicalAnalysis {
    pub original: String,
    pub state: u32,
    pub states: Vec<u32>,
    pub tokens: Vec<String>,
    pub idx: usize,
    pub bidx: usize,
}

impl LexicalAnalysis {
    pub fn new(original: &str) -> Self {
        Self {
            original: String::from(original),
            state: 0,
            states: Vec::new(),
            tokens: Vec::new(),
            idx: 0,
            bidx: 0,
        }
    }

    pub fn separate_token(&mut self, final_token: bool) {
        self.states.push(self.state);
        let sub = self.original.get(self.bidx..self.idx);
        match sub {
            Some(res) => self.tokens.push(String::from(res)),
            None => println!("Error"),
        }
        self.bidx = self.idx;
        match final_token {
            true => self.state = self.state,
            false => self.state = 0,
        }
    }

    pub fn compare(&self, idx: usize, lit: char) -> bool {
        let input_cp = self.original.clone();
        return input_cp.chars().nth(idx).unwrap() == lit;
    }

    pub fn check_for_types(&mut self, idx: usize) -> bool {
        let input_cp = self.original.clone();
        match input_cp.chars().nth(idx).unwrap() {
            'i' => {
                if self.compare(idx + 1, 'n')
                    && self.compare(idx + 2, 't')
                    && self.compare(idx + 3, ' ')
                {
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }

    pub fn analize(&mut self) {
        let cad_copy = self.original.clone(); //Avoid multiple unmutable/mutable borrow errors on self
        for i in cad_copy.chars() {
            match self.state {
                0 => {
                    // Initial state
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 3;
                    } else if i == ' ' || i == '\t' {
                        self.state = 6;
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
                    } else if i == ' ' || i == '\t' {
                        self.separate_token(true);
                        self.state = 6;
                    } else {
                        self.separate_token(false);
                    }
                }
                2 => {
                    //Numeric identifier
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 2;
                    } else if i == ' ' || i == '\t' {
                        self.separate_token(true);
                        self.state = 6;
                    } else {
                        self.separate_token(false);
                    }
                }
                3 => {
                    //Float number initial
                    if i.is_digit(10) {
                        self.state = 3;
                    } else if i == '.' {
                        self.state = 4;
                    } else if i == ' ' || i == '\t' {
                        self.separate_token(true);
                        self.state = 6;
                    } else {
                        self.separate_token(false);
                    }
                }
                4 => {
                    //Dot for Float number
                    if i.is_digit(10) {
                        self.state = 5;
                    } else {
                        self.separate_token(false);
                    }
                }
                5 => {
                    //Additional digits for Float number
                    if i.is_digit(10) {
                        self.state = 5;
                    } else if i == ' ' || i == '\t' {
                        self.separate_token(true);
                        self.state = 6;
                    } else {
                        self.separate_token(false);
                    }
                }
                6 => {
                    //Blank spaces
                    self.bidx = self.idx;
                    if i.is_alphabetic() {
                        self.state = 1;
                    } else if i.is_digit(10) {
                        self.state = 3;
                    } else if i == ' ' || i == '\t' {
                        self.state = 6;
                    } else {
                        self.state = u32::MAX;
                    }
                }
                _ => self.state = u32::MAX,
            }
            self.idx = self.idx + 1;
        }
        self.separate_token(true);
    }

    pub fn get_type(&self, state: u32) -> String {
        match state {
            0 => return String::from("Error"),
            1 | 2 => return String::from("Identificador"),
            3 => return String::from("Entero"),
            5 => return String::from("Número Real"),
            6 => return String::from("Espacio en blanco"),
            u32::MAX => String::from("Error de sintaxis"),
            _ => String::from("Error de sintaxis"),
        }
    }

    pub fn get_results(&self) {
        let mut i: usize = 0;
        for elem in &self.states {
            println!(
                "El token {} es de tipo {}",
                self.tokens[i],
                self.get_type(*elem)
            );
            i = i + 1;
        }
    }
}

impl Default for LexicalAnalysis {
    fn default() -> Self {
        Self {
            original: String::from(""),
            state: 0,
            states: Vec::new(),
            tokens: Vec::new(),
            idx: 0,
            bidx: 0,
        }
    }
}
