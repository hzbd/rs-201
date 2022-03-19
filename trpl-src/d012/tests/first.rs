
#[cfg(test)]
mod tests {
    use d012::*;

    #[test]
    fn find_word() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn digit_cmp1() {
        println!("this is demo test func");
        assert_eq!(2, 1);
    }

    #[test]
    fn digit_cmp2() {
        assert_eq!(5, add_two(5));
    }
}
