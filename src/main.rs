mod lexer;

use lexer::lexer;

fn main() {
    let source = "let abc = \"abc\";";

    let tokens = lexer(source);
    println!("{:?}", tokens);
}