mod token;
mod scanner;

pub use scanner::Scanner;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let s = "Hello World";
        let it: std::iter::Peekable<std::str::Chars> = s.chars().peekable();
    }
}
