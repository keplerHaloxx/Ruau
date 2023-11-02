mod lexer;
mod translator;

use lexer::lexer::tokenize;

fn main() {
    let code = r#"
let x = 10;
if x > 5 {
    println!("x is greater than 5");
} else {
    println!("x is less than or equal to 5");
}"#;

    let tokens = tokenize(code);

    for token in tokens {
        println!("{:?}", token);
    }
}
