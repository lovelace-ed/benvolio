#[proc_macro]
pub fn doctest_glob(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = if let Some(directory_tt) = input.into_iter().next() {
        match directory_tt {
            proc_macro::TokenTree::Literal(lit) => format!(
                "{}/{}",
                std::env::var("CARGO_MANIFEST_DIR").unwrap(),
                lit.to_string()
            ),
            _ => {
                return syn::Error::new(
                    proc_macro::Span::call_site().into(),
                    "Expected a string literal here.",
                )
                .to_compile_error()
                .into()
            }
        }
    } else {
        return syn::Error::new(
            proc_macro::Span::call_site().into(),
            "Expected a string literal here.",
        )
        .to_compile_error()
        .into();
    };
    let path = path.replace("\"", "");
    let mut output = quote::quote! {
        macro_rules! doc_comment {
            ($x:expr) => {
                #[doc = $x]
                extern {}
            };
            ($x:expr, $($tt:tt)*) => {
                #[doc = $x]
                $($tt)*
            };
        }
        macro_rules! doctest {
            ($x:expr) => {
                doc_comment!(include_str!($x));
            };
            ($x:expr, $y:ident) => {
                doc_comment!(include_str!($x), mod $y {});
            };
        }
    };
    for file in glob::glob(&path).expect("invalid file pattern") {
        let file = file.expect("error – invalid glob pattern");
        let str = file.as_os_str().to_str().unwrap();
        output = quote::quote! {
            #output

            doctest!(#str);
        };
    }
    output.into()
}
