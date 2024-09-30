pub fn hello() -> String {
    let from = String::from("Hello, world!");
    from
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_should_return_a_string(){
        hello();
    }
}

