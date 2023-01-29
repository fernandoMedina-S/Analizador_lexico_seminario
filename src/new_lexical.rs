pub struct LexicalAnalysis {
    original: String,
    tokens: Vec<String>,
    states: Vec<u32>,
    state: u32,
}

fn is_type(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "int" => true,
        "float" => true,
        "void" => true,
        _ => false,
    }
}

fn is_rel_type(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        ">" => true,
        "<" => true,
        "<=" => true,
        ">=" => true,
        _ => false,
    }
}

fn is_logic_op(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "&&" => 10,
        "||" => 11,
        "!" => 12,
        _ => 0,
    }
}

fn is_equal_op(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "==" => true,
        "!=" => true,
        _ => false,
    }
}

fn is_condictional_op(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "if" => 21,
        "else" => 22,
        _ => 0,
    }
}

fn is_reserved_word(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "while" => 23,
        "return" => 24,
        _ => 0,
    }
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
            if is_type(to_analize.clone()) {
                self.states.push(7);
                return;
            }
            if is_rel_type(to_analize.clone()) {
                self.states.push(9);
                return;
            }
            if is_logic_op(to_analize.clone()) != 0 {
                self.states.push(is_logic_op(to_analize.clone()));
                return;
            }
            if is_equal_op(to_analize.clone()) {
                self.states.push(13);
                return;
            }
            if is_condictional_op(to_analize.clone()) != 0 {
                self.states.push(is_condictional_op(to_analize.clone()));
                return;
            }
            if is_reserved_word(to_analize.clone()) != 0 {
                self.states.push(is_reserved_word(to_analize.clone()));
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

    fn get_type(&self, state: u32) -> String {
        match state {
            0 => return String::from("Error"),
            1 | 2 => return String::from("Identificador"),
            3 => return String::from("Entero"),
            5 => return String::from("Número Real"),
            6 => return String::from("Operador de suma/resta"),
            7 => return String::from("Tipo de dato"),
            8 => return String::from("Operador de Mult/Div"),
            9 => return String::from("Operador de relación"),
            10 => return String::from("Operador lógico AND"),
            11 => return String::from("Operador lógico OR"),
            12 => return String::from("Operador lógico NOT"),
            13 => return String::from("Operador de igualdad"),
            14 => return String::from("Operador de fin de intrucción"),
            15 => return String::from("Operador coma"),
            16 => return String::from("Abrir parentesis"),
            17 => return String::from("Cerrar parentesis"),
            18 => return String::from("Abrir llave"),
            19 => return String::from("Cerrar llave"),
            20 => return String::from("Operador de asignación"),
            21 => return String::from("Operador condicional"),
            22 => return String::from("Operador condicional else"),
            23 => return String::from("Ciclo condicional"),
            24 => return String::from("Retorno de función"),
            u32::MAX => String::from("Error de sintaxis"),
            _ => String::from("Error de sintaxis"),
        }
    }

    pub fn get_results(&self){
        let mut i: usize = 0;
        //println!("{:?}", self.states);
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