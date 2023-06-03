use test_palindroms;

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
    let mut r = Vec::with_capacity(cap * 2);
    for _ in 0..cap {
        r.push(std::hint::black_box(test_palindroms::pal_v1(s)));
    }
    println!("ended1");
    for _ in 0..cap {
        r.push(std::hint::black_box(test_palindroms::pal_v4(s)));
    }
    println!("ended2");
}
