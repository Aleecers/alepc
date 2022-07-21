use requestty::{prompt_one, ErrorKind, Question};

fn main() -> Result<(), ErrorKind> {
    let question = Question::input("name")
        .message("Hello what?")
        .default("World")
        .transform(|name, _, backend| write!(backend, "Hello, {}!", name))
        .build();
    prompt_one(question)?;
    Ok(())
}
