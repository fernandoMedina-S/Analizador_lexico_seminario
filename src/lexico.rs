pub struct Lexico {
    pub fuente: String,
    pub ind: u32,
    pub continua: bool,
    pub c: char,
    pub estado: u32,
    pub simbolo: String,
    pub tipo: u32,
}

impl Lexico {
    fn terminado(&self) -> bool {
        return self.ind >= (self.fuente.len() as u32);
    }
    
    fn sig_caracter(&self) -> char {
        if self.terminado() {
            return '$';
        }
        return self.fuente.chars().nth((self.ind + 1) as usize).unwrap();
    }

    fn sig_estado(&mut self, estado: u32) {
        self.estado = estado;
    }

    fn aceptacion(&mut self, estado: u32) {
        self.sig_estado(estado);
        self.continua = false;
    }

    fn es_letra(c: char) -> bool {
        return c.is_alphabetic();
    }

    fn es_numero(c: char) -> bool {
        return c.is_digit(10);
    }

    fn es_espacio(c: char) -> bool {
        return c == ' ' || c == '\t';
    }

    fn retroceso(&mut self) {
        if self.c != '$' {
            self.ind = self.ind-1;
        }
        self.continua = false;
    }

    pub fn entrada(&mut self ,fuente: String) {
        self.ind = 0;
        self.fuente = fuente;
    }

    pub fn sig_simbolo(&mut self) -> u32 {
        self.estado = 0;
        self.continua = true;
        self.simbolo = String::from("");

        while self.continua {
            self.c = self.sig_caracter();
            match self.estado {
                0 => if self.c == '+' || self.c == '-'{
                    self.aceptacion(2);
                }else{
                    if self.c == '$' {
                        self.aceptacion(3);
                    }
                },
                _ => print!("ño")
            }
        }

        self.tipo = match self.estado {
            2 => 2,
            _ => 3,
        };

        return self.tipo;
    }
    
}