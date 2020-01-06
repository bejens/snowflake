mod snowflake;


#[cfg(test)]
mod tests {
    use crate::snowflake::Snowflake;

    #[test]
    fn it_works() {
        let mut s = Snowflake::kubernetes();
        let id = s.generate().unwrap();
        println!("{}", id)
    }
}
