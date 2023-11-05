mod lexer;
mod translator;

use lexer::lexer::tokenize;
use crate::translator::translator::translate;

fn main() {
    let code = r#"
        fn main() {
            println!("hello");
            println!("yay");
        }

        fn hello() {
            println!("test");
        }
    "#;

    let tokens = tokenize(code);

    for token in &tokens {
        println!("{:?}", token);
    }

    println!("{}", translate(&tokens))
}
