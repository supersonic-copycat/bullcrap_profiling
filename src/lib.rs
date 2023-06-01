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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!pal_v1("Hello World!!"));
        assert!(pal_v1("a556roza u_@pala na , !!! lapu azora!"));
    }
}
