
pub fn word_for(n: u32) -> String {
    let mut string = String::new();

    if n % 3 == 0 {
        string.push_str("fizz ");
    }
    if n % 5 == 0 {
        string.push_str("buzz ");
    }

    if string.len() > 0 {
        string.pop();
        return string;
    }

    return n.to_string();
}

#[cfg(test)]
mod tests {
    use super::word_for;

    #[test]
    fn fizz_for_multiples_of_3() {
        assert_eq!("fizz", word_for(6));
    }

    #[test]
    fn buzz_for_multiples_of_5() {
        assert_eq!("buzz", word_for(10));
    }

    #[test]
    fn fizz_buzz_for_multiples_of_15() {
        assert_eq!("fizz buzz", word_for(15));
    }

    #[test]
    fn n_for_other_n() {
        assert_eq!("13", word_for(13));
    }
}
