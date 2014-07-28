fn facrec(n :uint) -> uint {
    if n <= 1 {
        return 1;
    } else {
    return n * facrec(n-1);

    }
}

fn fac(n :uint) -> uint {
    let mut i = 1;
    let mut result = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    return result;
}

#[test]
fn facrec_test() {
    assert!(120 == facrec(5));
}

#[test]
fn fac_test() {
    assert!(120 == fac(5));
}


