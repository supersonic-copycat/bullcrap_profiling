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
    let (len, mut seen) = (inp.chars().count(), 0usize);
    // empty strings and strings from 1 symbol are palindromes!
    if len < 2 {
        return true;
    }
    let (mut front, mut back) = (inp.chars(), inp.chars().rev());
    let (mut f, mut b) = (None, None);
    while seen < len / 2 {
        // on some pathological strings like "a@...lot of @'s...@a"
        // each loop will traverse all the @'s, when it's not needed actually
        // can move check seen < len / 2 inside
        loop {
            if let Some(c) = front.next() {
                seen += 1;
                if c.is_alphabetic() {
                    f = Some(c);
                    break;
                }
            } else {
                // when i reached and of the string f will retain it's last value!
                // probably it can fire in some conditions
                break;
            }
        }
        loop {
            if let Some(c) = back.next() {
                seen += 1;
                if c.is_alphabetic() {
                    b = Some(c);
                    break;
                }
            } else {
                break;
            }
        }
        if f != b {
            return false;
        }
    }
    return true;
}

pub fn pal_v5(inp: &str) -> bool {
    let mut direct = inp.chars().filter(|c| c.is_alphabetic());
    let mut reverse = inp.chars().rev().filter(|c| c.is_alphabetic());
    let len = direct.clone().count();
    let mut pos = 0usize;

    while pos < len / 2 {
        let a = direct.next();
        let b = reverse.next();
        if a != b {
            return false;
        }
        pos += 1;
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
        assert!(pal_v4(""));
        assert!(pal_v4("1"));
        assert!(pal_v4("22$"));
        assert!(pal_v4("a22$"));
        assert!(pal_v4("abba"));
        assert!(pal_v4("aba"));
        assert!(pal_v4("ab@ba"));
        assert!(pal_v4("22g$"));
        assert!(pal_v4("22g🤔$"));
        assert!(!pal_v4("a2🤡2g$"));
        assert!(pal_v4("🤦a22g$a."));
        assert!(pal_v4("*5,.{a22g$a.!!~]"));
        assert!(pal_v4("aba*5,.{22$.!!~]"));
        assert!(pal_v4("㊌㆞㊌"));
    }
}
