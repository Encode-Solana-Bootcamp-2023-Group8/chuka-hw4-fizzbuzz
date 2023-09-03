pub fn fizzbuzz() {
    let mut fc = Vec::new();
    for i in 1..=301 {
        if is_fizz_buzz(i) {
            fc.push(i);
            println!("{} is FizzBuzz", i);
        } else if is_fizz(i) {
            println!("{} is Fizz", i);
        } else if is_buzz(i) {
            println!("{} is Buzz", i);
        } else {
            println!("{}", i);
        }
    }
    println!("FizzBuzz count: {}", fc.len());
}

pub fn is_fizz_buzz(n: i32) -> bool {
if n % 15 == 0 {
    return true;
}
    false
}

pub fn is_fizz(n: i32) -> bool {
    if n % 3 == 0 {
        return true;
    }
    false
}

pub fn is_buzz(n: i32) -> bool {
    if n % 5 == 0 {
        return true;
    }
    false
}

fn main() {
    println!("Hello World");
    fizzbuzz();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_buzz() {
        assert_eq!(is_fizz(3), true);
        assert_eq!(is_fizz(5), false);
    }
    #[test]
    fn test_is_fizz() {
        assert_eq!(is_buzz(3), false);
        assert_eq!(is_buzz(5), true);
    }

    #[test]
    fn test_is_fizz_buzz() {
        assert_eq!(is_fizz_buzz(3), false);
        assert_eq!(is_fizz_buzz(5), false);
        assert_eq!(is_fizz_buzz(15), true);
    }
}