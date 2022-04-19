use proc_macro::{TokenStream, TokenTree};

pub fn html(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    let html = if let Some(TokenTree::Literal(literal)) = tokens.next() {
        let repr = literal.to_string();
        let repr = repr.trim();

        if repr.starts_with('"') || repr.starts_with('r') {
            let begin = repr.find('"').unwrap() + 1;
            let end = repr.rfind('"').unwrap();

            let open = &repr[..begin];
            let (format, args) = split(&repr[begin..end]);
            let close = &repr[end..];

            format!("Html(format!({open}{format}{close}{args}))")
        } else {
            panic!("invalid html invocation: argument must be a single string literal")
        }
    } else {
        panic!("invalid html invocation: argument must be a single string literal")
    };

    assert!(tokens.next().is_none());

    html.parse().expect("failed to parse tokens")
}

fn split(content: &str) -> (String, String) {
    let mut format = String::new();
    let mut args = String::new();

    let mut pos = 0;
    while let Some(index) = content[pos..].find('{').map(|index| index + pos) {
        let close = content[pos..].find('}').expect("no closing bracket") + pos;
        format = format + &content[pos..=index] + "}";
        args = args + ", " + &content[index + 1..close] + ".to_html()";
        pos = close + 1;
    }
    format += &content[pos..];

    (format, args)
}
