use crate::lexer::token::*;

pub fn print_error(file: &str, line: i32, column: i32, header: &str, description: &str, error_code: &str, error_token: Token, caption: &str, notes: Vec<&str>, suggestion: &str, suggested_code: &str) {
  println!("\nError -> {} | {}:{}", file, line, column);
  println!("{}\n", header);

  println!("{}\n", description);

  let len = line.to_string().len();
  for _ in 0..len + 1 {
    print!(" ");
  }

  println!("|");

  println!("{} | {}", line, error_code);

  for _ in 0..len + 1 {
    print!(" ");
  }

  print!("|");

  for _ in 0..error_token.pos {
    print!(" ");
  }

  for _ in 0..error_token.content.len() {
    print!("^");
  }

  println!(" <- {}\n", caption);

  for n in notes.clone() {
    println!("note: {}", n);
  }

  if notes.len() > 0 { println!() }

  println!("Suggestion:\n{}\n", suggestion);

  for _ in 0..len + 1 {
    print!(" ");
  }

  println!("|");

  println!("{} | {}", line, suggested_code);

  for _ in 0..len + 1 {
    print!(" ");
  }

  print!("| ");

  for _ in 0..suggested_code.len() {
    print!("+");
  }

  println!()

}
