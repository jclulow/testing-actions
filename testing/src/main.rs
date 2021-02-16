fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test0() {
        let mut abc = String::new();
        abc += "a";
        abc += "b";
        abc += "c";
        assert_eq!("abc", abc);
    }
}
