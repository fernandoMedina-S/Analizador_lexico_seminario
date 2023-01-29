pub fn is_type(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "int" => true,
        "float" => true,
        "void" => true,
        _ => false,
    }
}

pub fn is_rel_type(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        ">" => true,
        "<" => true,
        "<=" => true,
        ">=" => true,
        _ => false,
    }
}

pub fn is_logic_op(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "&&" => 10,
        "||" => 11,
        "!" => 12,
        _ => 0,
    }
}

pub fn is_equal_op(for_analize: String) -> bool {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "==" => true,
        "!=" => true,
        _ => false,
    }
}

pub fn is_condictional_op(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "if" => 21,
        "else" => 22,
        _ => 0,
    }
}

pub fn is_reserved_word(for_analize: String) -> u32 {
    let s_slice: &str = &*for_analize;
    match s_slice {
        "while" => 23,
        "return" => 24,
        _ => 0,
    }
}

pub fn get_numerical_value(state: u32) -> u32 {
    match state {
        0 => return u32::MAX,
        1 | 2 => return 0,
        3 => return 1,
        5 => return 2,
        6 => return  5,
        7 => return  4,
        8 => return  6,
        9 => return  7,
        10 => return  9,
        11 => return  8,
        12 => return  10,
        13 => return  11,
        14 => return  12,
        15 => return  13,
        16 => return  14,
        17 => return  15,
        18 => return  16,
        19 => return  17,
        20 => return  18,
        21 => return  19,
        22 => return  22,
        23 => return  20,
        24 => return  21,
        u32::MAX =>  u32::MAX,
        _ => u32::MAX,
    }
}

pub fn get_type(state: u32) -> String {
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