fn main() {
    println!("Hello, world!");
}

// #[test]
// fn test_parser() {
//     let language = unsafe {tree_sitter_python()};
//     let mut parser = Parser::new();
//     parser.set_language(language).unwrap();

//     let source_code = "print('hello there')";
//     let tree = parser.parse(source_code, None).unwrap();

//     assert_eq!(tree.root_node().to_sexp(), "(source_file ())")
// }
