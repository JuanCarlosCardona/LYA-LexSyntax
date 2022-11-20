use crate::{Lex, Token};
use crate::Token::{DELIMITER, IDENTIFIER, LexError, LITERAL, LogicalOperator};

/*
Grammar:

if (expr) {instr} else {instr}

 */
pub fn parser(symbol_table: Vec<Lex>) -> String {


    if symbol_table.get(0).unwrap().lexeme != String::from("if"){

        return "Syntax error: Keyword expected at line ".to_owned() + symbol_table.get(0).unwrap().line.to_string().as_ref();
    }

    if symbol_table.get(1).unwrap().lexeme != String::from("("){
        return "Syntax error at line ".to_owned() + symbol_table.get(1).unwrap().line.to_string().as_ref();
    }

    if symbol_table.get(2).unwrap().lexeme == String::from(")"){
        return "Syntax error: Expression expected at line ".to_owned() + symbol_table.get(2).unwrap().line.to_string().as_ref();
    }

    let mut previous_lex: &Lex = symbol_table.get(1).unwrap();
    let mut lex = symbol_table.get(2).unwrap();
    let mut index = 2;
    let mut is_analyzing_expression = false;

    while lex.lexeme != String::from(")") && index < symbol_table.len(){

        if lex.token == LexError {
            return "Unrecognized at line ".to_owned() + lex.line.to_string().as_ref();
        }

        match previous_lex.token {
            DELIMITER => {
                if previous_lex.lexeme == String::from("("){
                    is_analyzing_expression = true;

                    match lex.token {
                        IDENTIFIER|Token::NUMBER|LITERAL => {

                            if lex.token == LITERAL {

                                let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                                if literal.len() < 2 {
                                    return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                            }

                            index += 1;
                            previous_lex = lex;
                        }

                        _ => {
                            return String::from("Syntax error at line ".to_owned()
                                + lex.line.to_string().as_ref());
                        }
                    }
                }
            }

            IDENTIFIER|Token::NUMBER|LITERAL =>{
                if is_analyzing_expression {

                    if lex.token != LogicalOperator{
                        println!("{}",lex.lexeme);
                        return String::from("Syntax error: Logical operator expected at line ".to_owned() +
                            lex.line.to_string().as_ref());
                    }

                    index += 1;
                    previous_lex = lex;
                }
                else if lex.token == IDENTIFIER{
                    match lex.token {
                        Token::AssignmentOperator|Token::ArithmeticOperator|DELIMITER => {

                            if lex.token == DELIMITER
                                && lex.lexeme != String::from(";")
                                && !is_analyzing_expression{
                                return String::from("Syntax error at line ".to_owned()
                                    + lex.line.to_string().as_ref());
                            }


                            index += 1;
                            previous_lex = lex;
                        }

                        _ => {
                            return String::from("Syntax error at line ".to_owned()
                                + lex.line.to_string().as_ref());
                        }
                    }
                }
            }

            LogicalOperator => {
               match lex.token {
                   IDENTIFIER|Token::NUMBER|LITERAL => {
                       if lex.token == LITERAL {

                           let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                           if literal.len() < 2 {
                               return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                           }

                       }
                        index += 1;
                        previous_lex = lex;
                   }

                   _ => {
                       return String::from("Syntax error at line ".to_owned() + lex.line.to_string().as_ref());
                   }
               }
            }

            _ => {index += 1;}
        }
        lex = symbol_table.get(index).unwrap();
    }

    if symbol_table.get(index+1).unwrap().lexeme != String::from("{"){
        return "Syntax error: Body expected at line ".to_owned() + symbol_table.get(index).unwrap().line.to_string().as_ref();
    }

    index += 2;
    lex = symbol_table.get(index).unwrap();
    previous_lex = symbol_table.get(index - 1).unwrap();

    while lex.lexeme != String::from("}") && index < symbol_table.len() {

        if lex.token == LexError{
            return "Unrecognized sequence at line ".to_owned() + lex.line.to_string().as_ref();
        }

        match previous_lex.token {
            DELIMITER =>{

                if previous_lex.lexeme == String::from("{"){

                    match lex.token {
                        IDENTIFIER|Token::NUMBER|LITERAL => {

                            if lex.token == LITERAL {

                                let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                                if literal.len() < 2 {
                                    return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                            }

                            index += 1;
                            previous_lex = lex;
                        }
                        _ => {return "Syntax error at line ".to_string() + lex.line.to_string().as_ref();}
                    }
                }

                if previous_lex.lexeme == String::from(";"){

                    match lex.token {
                        IDENTIFIER|Token::NUMBER|LITERAL => {
                            if lex.token == LITERAL {

                                let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                                if literal.len() < 2 {
                                    return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                            }

                            index += 1;
                            previous_lex = lex;

                            match symbol_table.get(index).unwrap().token {
                                IDENTIFIER|Token::NUMBER|LITERAL => {
                                    return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                                DELIMITER => {
                                    if symbol_table.get(index).unwrap().lexeme != String::from(";"){
                                        return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref() ;
                                    }
                                }
                                 _ => {}
                            }

                        }
                        _ => {return "Syntax error at line ".to_string() + lex.line.to_string().as_ref()}
                    }

                }

            }

            IDENTIFIER|Token::NUMBER|LITERAL => {

                match lex.token {

                    LogicalOperator|Token::AssignmentOperator|Token::ArithmeticOperator|DELIMITER => {

                        if lex.token == DELIMITER && lex.lexeme != String::from(";"){
                            return "Syntax Error at line ".to_owned() + lex.line.to_string().as_ref();
                        }

                        index += 1;
                        previous_lex = lex;
                    }
                    IDENTIFIER|Token::NUMBER|LITERAL => {
                        return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                    }

                    _ => {}
                }
            }

            LogicalOperator|Token::ArithmeticOperator|Token::AssignmentOperator => {

                match lex.token {
                    IDENTIFIER|Token::NUMBER|LITERAL => {
                        index += 1;
                        previous_lex = lex;

                        if symbol_table.get(index).unwrap().lexeme != String::from(";") {
                           return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                        }

                    }

                    _ => {}
                }
            }

             _ => {}
        }

        if index + 1 == symbol_table.len() && symbol_table.get(index).unwrap().lexeme != String::from("}"){
            return "Syntax Error: Unclosed body at line ".to_owned() + lex.line.to_string().as_ref();
        }

        lex = symbol_table.get(index).unwrap();
    }

    if symbol_table.get(index).unwrap().lexeme != String::from("}") {
        return "Syntax Error: Unclosed body".to_owned() + symbol_table.get(index).unwrap().line.to_string().as_ref();
    }

    if index + 1 == symbol_table.len() {
        return "Ok!".to_string();
    }

    index += 1;
    lex = symbol_table.get(index).unwrap();

    if lex.lexeme != String::from("else") {
        return "Syntax error: else keyword expected at line ".to_owned() + lex.line.to_string().as_ref();
    }

    index += 1;

    if symbol_table.get(index).unwrap().lexeme != String::from("{"){
        return "Syntax Error: Body expected at line ".to_owned() + symbol_table.get(index).unwrap().line.to_string().as_ref();
    }

    previous_lex = symbol_table.get(index).unwrap();
    index += 1;
    lex = symbol_table.get(index).unwrap();


    while lex.lexeme != String::from("}") && index < symbol_table.len(){

        if index + 1 == symbol_table.len(){
            return "Syntax Error: Unclosed body at line ".to_owned() + lex.line.to_string().as_ref();
        }

        if lex.token == LexError{
            return "Unrecognized sequence at line ".to_owned() + lex.line.to_string().as_ref();
        }

        match previous_lex.token {
            DELIMITER =>{

                if previous_lex.lexeme == String::from("{"){

                    match lex.token {
                        IDENTIFIER|Token::NUMBER|LITERAL => {
                            if lex.token == LITERAL {

                                let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                                if literal.len() < 2 {
                                    return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                            }

                            index += 1;
                            previous_lex = lex;
                        }
                        _ => {return "Syntax error at line ".to_string() + lex.line.to_string().as_ref();}
                    }
                }

                if previous_lex.lexeme == String::from(";"){

                    match lex.token {
                        IDENTIFIER|Token::NUMBER|LITERAL => {

                            if lex.token == LITERAL {

                                let literal: Vec<&str> = lex.lexeme.matches("\"").collect();

                                if literal.len() < 2 {
                                    return "Syntax Error: Missing \" at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                            }

                            index += 1;
                            previous_lex = lex;

                            match symbol_table.get(index).unwrap().token {
                                IDENTIFIER|Token::NUMBER|LITERAL => {
                                    return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                                }

                                DELIMITER => {
                                    if symbol_table.get(index).unwrap().lexeme != String::from(";"){
                                        return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                                    }
                                }
                                _ => {}
                            }

                        }
                        _ => {return "Syntax error at line ".to_owned() + lex.line.to_string().as_ref()}
                    }

                }

            }

            IDENTIFIER|Token::NUMBER|LITERAL => {

                match lex.token {

                    LogicalOperator|Token::AssignmentOperator|Token::ArithmeticOperator|DELIMITER => {

                        if lex.token == DELIMITER && lex.lexeme != String::from(";"){
                            return "Syntax Error at line ".to_owned() + lex.line.to_string().as_ref();
                        }

                        index += 1;
                        previous_lex = lex;
                    }
                    IDENTIFIER|Token::NUMBER|LITERAL => {
                        return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                    }

                    _ => {}
                }
            }

            LogicalOperator|Token::ArithmeticOperator|Token::AssignmentOperator => {

                match lex.token {
                    IDENTIFIER|Token::NUMBER|LITERAL => {
                        index += 1;
                        previous_lex = lex;

                        if symbol_table.get(index).unwrap().lexeme != String::from(";") {
                            return "Syntax Error: Missing ; at line ".to_owned() + lex.line.to_string().as_ref();
                        }

                    }

                    _ => {}
                }
            }

            _ => {}
        }
        lex = symbol_table.get(index).unwrap();

    }

    "Ok!".to_string()
}



