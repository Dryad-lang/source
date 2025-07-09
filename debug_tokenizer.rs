use crate::lexer::tokenizer::Lexer;

fn main() {
    let code = "namespace Test { }";
    let mut lexer = Lexer::new(code);
    
    loop {
        let token = lexer.next_token();
        println!("Token: {:?}", token);
        if matches!(token, Token::Eof) {
            break;
        }
    }
}
