
fn is_valid(num: i32, strict_single_dup: bool) -> bool {
    let digits: Vec<i32> = (0..6).rev().map(|i| (num / 10i32.pow(i)) % 10).collect();
    let mut has_dup = false;
    for i in 0..5 {
        if digits[i] > digits[i+1] {
            return false;
        }
        if digits[i] == digits[i+1] {
            if !strict_single_dup || ((i == 0 || digits[i-1] != digits[i]) && (i == 4 || digits[i+2] != digits[i+1])) {
                has_dup = true;
            }
        }
    }
    return has_dup;
}

#[test]
fn test_simple() {
    assert!(is_valid(111111, false));
    assert!(!is_valid(223450, false));
    assert!(!is_valid(123789, false));

    assert!(is_valid(112233, true));
    assert!(!is_valid(123444, true));
    assert!(is_valid(111122, true));
}

fn main() {
    let res_1 = (136818..=685979i32).filter(|x| is_valid(*x, false)).count();
    let res_2 = (136818..=685979i32).filter(|x| is_valid(*x, true)).count();
    println!("{}, {}", res_1, res_2);
}
