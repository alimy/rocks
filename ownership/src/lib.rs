
pub fn run() {
    let s = String::from("hello");
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
