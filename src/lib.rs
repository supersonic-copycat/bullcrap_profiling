pub fn pal_v1(inp: &str) -> bool {
    let direct = inp.chars().filter(|c| c.is_alphabetic());
    let reverse = inp.chars().filter(|c| c.is_alphabetic()).rev();
    for (a, b) in direct.zip(reverse) {
        if a != b {
            return false;
        }
    }
    return true;
}

pub fn pal_v2(inp: &str) -> bool {
    let direct = inp.chars().filter(|c| c.is_alphabetic());
    let reverse = direct.clone().rev();
    for (a, b) in direct.zip(reverse) {
        if a != b {
            return false;
        }
    }
    return true;
}

pub fn pal_v3(inp: &str) -> bool {
    let direct = inp.chars().filter(|c| c.is_alphabetic());
    let reverse = direct.clone().rev();
    let len = direct.clone().count() / 2;
    for (i, (a, b)) in direct.zip(reverse).enumerate() {
        if i > len {
            break;
        }
        if a != b {
            return false;
        }
    }
    return true;
}

pub fn pal_v4(inp: &str) -> bool {
    // &str is utf-8, so I can't index it. Iterating is possble, but it will be n^2
    // each time we request ith char we have to walk from beginning!
    // So solution would be using iterators directly. Also, need to split by graphemes, not chars
    let len = inp.chars().count();
    // empty strings and strings from 1 symbol is palindrome!
    if len < 2 {
        return true;
    }
    let mut seen = 0usize;
    let mut front = inp.chars();
    let mut back = inp.chars().rev();
    let (mut f, mut b) = (None, None);
    while seen < len / 2 {
        loop {
            match front.next() {
                Some(c) => {
                    seen += 1;
                    if c.is_alphabetic() {
                        f = Some(c);
                        break;
                    }
                }
                None => {
                    break;
                }
            };
        }
        loop {
            match back.next() {
                Some(c) => {
                    seen += 1;
                    if c.is_alphabetic() {
                        b = Some(c);
                        break;
                    }
                }
                None => {
                    break;
                }
            };
        }
        if f != b {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_works() {
        assert!(!pal_v1("Hello World!!"));
        assert!(pal_v1("a556roza u_@pala na , !!! lapu azora!"));
    }
    #[test]
    fn v2_works() {
        assert!(!pal_v2("Hello World!!"));
        assert!(pal_v2("a556roza u_@pala na , !!! lapu azora!"));
    }
    #[test]
    fn v3_works() {
        assert!(!pal_v3("Hello World!!"));
        assert!(pal_v3("a556roza u_@pala na , !!! lapu azora!"));
    }
    #[test]
    fn v4_works() {
        assert!(!pal_v4("Hello World!!"));
        assert!(pal_v4("a556roza u_@pala na , !!! lapu azora!"));
    }
}
