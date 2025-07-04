use std::borrow::Cow;

fn escape<'a>(input: &'a str) -> Cow<'a, str> {
    // ' => \'
    // " => \"
    // foo => foo

    if escape_not_required(input) {
        Cow::Borrowed(input)
    }
    else {
        let mut s = input.to_string();
        //do something to s
        Cow::Owned(s)
    }
}

fn escape_not_required(input: &str) -> bool {
    false
}