use test_palindroms;

fn run(s: &str, f: Box<dyn Fn(&str) -> bool>) -> bool {
    f(s)
}

fn present(b: bool) -> &'static str {
    match b {
        true => "",
        false => "not",
    }
}

fn main() {
    let s = "hello, world!";
    println!(
        "so far so good: \"{0}\" is {1} palindrom!",
        s,
        present(run(s, Box::new(test_palindroms::pal_v1)))
    );
}
