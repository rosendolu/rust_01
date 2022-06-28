fn main() {
    let a = "Здравствуйте";
    let b = "234234";

    for b2 in a.chars() {
        dbg!(b2);
    }

    for b2 in a.bytes() {
        dbg!(b2);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn echo_test() {
        assert_eq!(1, 1);
        dbg!("test");
    }
}
