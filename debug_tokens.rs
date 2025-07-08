// debug_tokens.rs
// Programa para debugar tokens

use dryad::lexer::tokenizer::Lexer;

fn main() {
    let code = r#"foo("bar");"#;
    println!("Debug dos tokens para: {}", code);
    
    let mut lexer = Lexer::new(code);
    
    loop {
        let token = lexer.next_token();
        println!("Token: {:?}", token);
        
        if matches!(token, dryad::lexer::token::Token::Eof) {
            break;
        }
    }
}
