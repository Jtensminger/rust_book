#[cfg(test)]
mod tests {
    #[test]
    fn add_two () -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn three_three_add () -> Result<(), String> {
        if 3 + 3 == 6 {
            Ok(())
        } else {
            Err(String::from("three plus three does not equal six"))
        }
    }

    #[test]
    fn four_two_multiply () -> Result<(), String> {
        if 4 * 2 == 8 {
            Ok(())
        } else {
            Err(String::from("four * two does not equal eight"))
        }
    }
}
