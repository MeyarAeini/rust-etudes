pub fn strtok<'b>(input: &mut &'b str, delimiter: char) -> &'b str {
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

//Invariants 
//
//fn foo(x: &mut &'a str, y: &'a str) {
//      *x = &y;
//}
//
//let mut x:&'static str = "hello world";
//let y = String::new("some string");
//foo(&mut x, &y);
//drop(y);
//println!("{}", x);
