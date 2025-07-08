pub fn strtok<'a>(input: &'a mut &'a str, delimiter: char) -> &str {
    if let Some(i) = input.find(delimiter) {
        let prefix = &input[..i];
        let suffix = &input[(i + delimiter.len_utf8())..];
        *input = suffix;

        prefix
    } else {
        let prefix = *input;
        *input = "";

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = "hello world";
        let hello = strtok(&mut input, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(input, "world");
    }
}
