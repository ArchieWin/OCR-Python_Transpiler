enum LineTypes {
    STRING,
    INT,
    FLOAT,
    REAL,
    BOOLEAN,
    ASSIGNMENTORFUNCTIONCALL,
}

pub fn translate_line(line: &String) -> String {

    let mut tokens = line.split_whitespace();

    let first_token = match tokens.next() {
        Some(value) => value,
        None => &"".to_string(),
    };

    let line_type = match_first_token_to_line_type(first_token);



    let translated_str = line_type.translate_line(line);

    println!("Translted String: {}", translated_str);

    return translated_str;

}

impl LineTypes {
    fn translate_line(&self, line : &String) -> String {
        let translated_line = match &self {

            LineTypes::STRING => translate_variable_init_line(line),
            LineTypes::INT => translate_variable_init_line(line),
            LineTypes::FLOAT => translate_variable_init_line(line),
            LineTypes::REAL => translate_variable_init_line(line),
            LineTypes::BOOLEAN => translate_variable_init_line(line),
            LineTypes::ASSIGNMENTORFUNCTIONCALL => line.clone(),

        };

        return translated_line;
    }
}

fn translate_variable_init_line(line : &String) -> String {

    let mut tokens = line.split(" ");
    let mut translated_line = String::new();

    tokens.next();
    translated_line.push_str(tokens.next().unwrap());

    for token in tokens {
        translated_line.push_str(" ");
        translated_line.push_str(token);
    }

    return translated_line;
}

fn match_first_token_to_line_type(token : &str) -> LineTypes {
    let result = match token {
        "String" => LineTypes::STRING,
        "int" => LineTypes::INT,
        "float" => LineTypes::FLOAT,
        "real" => LineTypes::REAL,
        "boolean" => LineTypes::BOOLEAN,
        _ => LineTypes::ASSIGNMENTORFUNCTIONCALL,
    };

    return result;
}

