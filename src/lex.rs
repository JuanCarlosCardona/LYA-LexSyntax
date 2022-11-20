use std::fmt;
use std::fmt::{Formatter};
use regex::{Regex, RegexSet};
use crate::lex::Token::KEYWORD;

pub struct Lex {
    pub(crate) lexeme: String,
    pub(crate) token: Token,
    pub(crate) pattern: String,
    pub(crate) line: usize
}

#[derive(PartialEq, Eq)]
pub enum Token{
    IDENTIFIER = 0,
    KEYWORD = 1,
    DELIMITER = 2,
    ArithmeticOperator = 3,
    AssignmentOperator = 4,
    LogicalOperator= 5,
    NUMBER = 6,
    LITERAL = 7,
    LexError = 8
}

pub enum Pattern {
    IDENTIFIER = 1,
    DELIMITER = 2,
    ArithmeticOperator = 3,
    AssignmentOperator = 4,
    LogicalOperator = 5,
    NUMBER = 6,
    LITERAL = 7,
    LexError = 8
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       match self {
           Pattern::IDENTIFIER => write!(f,"Letter followed by letters and digits"),
           Pattern::DELIMITER => write!(f, "() {{}} [] ; ."),
           Pattern::ArithmeticOperator => write!(f, "+ - * / %"),
           Pattern::AssignmentOperator => write!(f, "= += -= *= /= %= |= &= ^="),
           Pattern::LogicalOperator => write!(f, "|| && == ! != < > <= >= "),
           Pattern::NUMBER => write!(f, "Any numeric constant"),
           Pattern::LITERAL => write!(f, "Anything except for \" surrounded for \"s"),
           Pattern::LexError => write!(f, "Unrecognized sequence")
       }
    }
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       match self {
           Token::IDENTIFIER => write!(f, "IDENTIFIER"),
           KEYWORD => write!(f, "KEYWORD"),
           Token::DELIMITER => write!(f, "DELIMITER"),
           Token::ArithmeticOperator => write!(f, "ARITHMETIC OPERATOR"),
           Token::AssignmentOperator => write!(f, "ASSIGNMENT OPERATOR"),
           Token::LogicalOperator => write!(f, "LOGICAL OPERATOR"),
           Token::NUMBER => write!(f, "NUMBER"),
           Token::LITERAL => write!(f, "LITERAL"),
           Token::LexError => write!(f, "LEX ERROR")
       }
    }
}


fn arithmetic_operator(mut current_index: usize,
                       mut current_character: String, next_character: &char, symbol_table:
                       &mut Vec<Lex>,
                       n_line: usize) -> usize{

    if *next_character == '='{
        current_character.push(*next_character);
        symbol_table.push(Lex{lexeme: current_character.to_string(),
                           token: Token::AssignmentOperator,
            pattern: Pattern::AssignmentOperator.to_string(),
            line: n_line
        });
        current_index += 2;
    }
    else {
        symbol_table.push(Lex{lexeme: current_character.to_string(),
                           token: Token::ArithmeticOperator, pattern: Pattern::ArithmeticOperator.to_string(),
            line: n_line
        });
        current_index += 1;
    }

    current_index
}

fn logical_operator(mut current_index: usize, mut current_char: String, next_character: &char,
                    symbol_table: &mut Vec<Lex>,
                    n_line: usize) -> usize {

    match current_char.as_str() {
        "!"|"<"|">"|"=" => {

            if *next_character == '=' {
                current_char.push(*next_character);
                current_index += 2;
            }
            else {
                current_index += 1
            }
            symbol_table.push(Lex{lexeme: current_char,
                token: Token::LogicalOperator,
                pattern: Pattern::LogicalOperator.to_string(),
                line: n_line
            });
        }

        _ => {
            if *next_character == '=' {
                current_char.push(*next_character);
                current_index += 2;
                symbol_table.push(Lex{lexeme: current_char,
                    token: Token::AssignmentOperator,
                    pattern: Pattern::AssignmentOperator.to_string(),
                    line: n_line
                });

                return current_index
            }
            else if next_character.to_string() == current_char {
                current_char.push(*next_character);
                current_index += 2;
            }
            else {
                current_index += 1;
            }

            symbol_table.push(Lex{lexeme: current_char.to_string(),
                token: Token::LogicalOperator,
                pattern: Pattern::LogicalOperator.to_string(),
                line: n_line
            });
        }
    }

    current_index
}

fn equals_operator(mut current_index: usize, mut current_char: String, next_char: &char, symbol_table:
&mut Vec<Lex>,
                   n_line: usize) -> usize{

    if next_char.to_string() == current_char {
        current_char.push(*next_char);
        current_index += 2;
        symbol_table.push(Lex{lexeme: current_char,
            token: Token::LogicalOperator,
            pattern: Pattern::LogicalOperator.to_string(),
            line: n_line
        });
    }else {
        current_index += 1;
        symbol_table.push(Lex{lexeme: current_char,
            token: Token::AssignmentOperator,
            pattern: Pattern::AssignmentOperator.to_string(),
            line: n_line
        });
    }

    current_index
}


fn identifier(current_index: usize, mut current_char: String, symbol_table: &mut Vec<Lex>, line: &Vec<char>, n_line: usize) -> usize {

    let keyword_identifier_regex = RegexSet::new(&[
        r"if|else",
        r"\w"
    ]).unwrap();

    let mut next_index = current_index + 1;

    if next_index == line.len(){
        symbol_table.push(Lex{lexeme: current_char, token:Token::IDENTIFIER, pattern: Pattern::IDENTIFIER.to_string(), line: n_line });
        return next_index;
    }


    let mut next_char = line.get(next_index).unwrap();

    while keyword_identifier_regex.matches(next_char.to_string().as_ref()).matched(1) &&
        next_index < line.len(){

        current_char.push(*next_char);

        if keyword_identifier_regex.matches(current_char.as_str().as_ref()).matched(0) {
            symbol_table.push(Lex{lexeme: current_char.as_str().parse().unwrap(),
                token: KEYWORD,
                pattern: current_char.as_str().parse().unwrap(),
                line: n_line
            });
            return next_index + 1;
        }

        next_index += 1;
        next_char = line.get(next_index).unwrap();
    }

    symbol_table.push(Lex{lexeme: current_char,
        token: Token::IDENTIFIER,
        pattern: Pattern::IDENTIFIER.to_string(),
        line: n_line
    });
    return next_index;
}

fn number(current_index: usize, mut current_char: String,
          symbol_table: &mut Vec<Lex>, line: &Vec<char>,
          n_line: usize) -> usize{

    let re = Regex::new(r"\d").unwrap();
    let mut next_index = current_index + 1;
    let mut next_char = line.get(next_index).unwrap();

    if !re.is_match(next_char.to_string().as_ref()){
        symbol_table.push(Lex{lexeme: current_char,
            token: Token::NUMBER,
            pattern: Pattern::NUMBER.to_string(),
            line: n_line
        });
        return  next_index;
    }

    while re.is_match(next_char.to_string().as_ref()) && next_index < line.len(){
        current_char.push(*next_char);

        next_index += 1;

        if next_index == line.len() {break;}

        next_char = line.get(next_index).unwrap();
    }

    symbol_table.push(Lex{lexeme: current_char,
        token: Token::NUMBER,
        pattern: Pattern::NUMBER.to_string(),
        line: n_line
    });
    next_index
}

fn literal(current_index: usize, mut current_char: String,
           symbol_table: &mut Vec<Lex>, line: &Vec<char>,
           n_line: usize) -> usize {

    let mut next_index = current_index + 1;
    let mut next_char = line.get(next_index).unwrap();
    let re = Regex::new(r"\w").unwrap();

    if next_char == &'"' {
        return next_index;
    }

    while re.is_match(next_char.to_string().as_ref()) && next_index < line.len(){
        current_char.push(*next_char);

        next_index += 1;
        next_char = line.get(next_index).unwrap();
    }

    current_char.push(*line.get(next_index).unwrap());

    symbol_table.push(Lex{lexeme: current_char,
        token: Token::LITERAL,
        pattern: Pattern::LITERAL.to_string(),
        line: n_line
    });
    next_index + 1
}

pub fn lex_analyzer(line: String, symbol_table: &mut Vec<Lex>, n_line: usize) {

    let _regex = RegexSet::new(&[
        r"[+]|-|[*]|/|%",
        r"[<>|&!]",
        r"[(){}\[\];.]",
        r"^=",
        r"[a-z]|[A-z]",
        r"^\d$",
        r###"""###,
        r"\s"
    ]).unwrap();

    let length = line.len();
    let mut index = 0;
    let line_chars: Vec<char> = line.chars().collect();

    while index < length {
        let mut current_char = line_chars.get(index).unwrap().to_string();

        let mut collect_matched: Vec<_> = _regex.matches(current_char.to_string().as_ref())
            .iter()
            .collect();

        if collect_matched.len() == 0{
            symbol_table.push(Lex{lexeme: current_char,
                token: Token::LexError,
                pattern: Pattern::LexError.to_string(),
                line: n_line
            });
            index += 1;
            current_char = line_chars.get(index).unwrap().to_string();
            collect_matched = _regex.matches(current_char.to_string().as_ref())
                .iter()
                .collect();
        }

        match collect_matched.get(0).unwrap() {

            0 => {
                let next_char = line_chars.get(index +1 ).unwrap();
                index = arithmetic_operator(index, current_char,
                                    next_char, symbol_table, n_line);
            }

            1 => {
                let next_char = line_chars.get(index + 1).unwrap();
                index = logical_operator(index, current_char, next_char,
                                         symbol_table, n_line);
            }

            2 => {
                symbol_table.push(Lex{lexeme: current_char,
                    token: Token::DELIMITER,
                    pattern: Pattern::DELIMITER.to_string(),
                    line: n_line
                });
                index += 1;
            }

            3 => {
                let next_char = line_chars.get(index + 1).unwrap();

                index = equals_operator(index, current_char, next_char, symbol_table, n_line);
            }

            4 => {
                index = identifier(index, current_char, symbol_table, line_chars.as_ref(), n_line);
            }

            5 => {
                index = number(index, current_char, symbol_table, line_chars.as_ref(), n_line);
            }

            6 => {
                index = literal(index, current_char, symbol_table, line_chars.as_ref(),
                n_line);
            }

            7 => {
                index += 1;
            }

            _ => {
                symbol_table.push(Lex{lexeme: current_char,
                    token: Token::LexError, pattern:
                    Pattern::LexError.to_string(),
                    line: n_line
                });
                index += 1;
            }

        }
    }
}