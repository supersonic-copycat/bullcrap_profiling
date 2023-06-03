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
    let s = "a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!";
    let cap = 10_000;
    let mut r = Vec::with_capacity(cap);
    for _ in 0..cap {
        r.push(std::hint::black_box(run(s, &test_palindroms::pal_v1)));
    }
}
