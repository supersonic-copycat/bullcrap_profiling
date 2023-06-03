use test_palindroms;

fn run(s: &str, f: &dyn Fn(&str) -> bool) -> bool {
    f(s)
}

fn present(s: &str, b: bool) -> String {
    format!(
        "\"{0}\" is {1} palindrom.",
        s,
        match b {
            true => "",
            false => "not",
        }
    )
}

fn main() {
    let s = "hi,ih!";
    println!(
        "so far so good.\n{}",
        present(s, run(s, &test_palindroms::pal_v1))
    );
}
