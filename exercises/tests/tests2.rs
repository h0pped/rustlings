// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
struct Test {
    a: i32,
}

#[cfg(test)]
mod tests {
    use crate::Test;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(Test { a: 1 }, Test { a: 1 });
    }
}
