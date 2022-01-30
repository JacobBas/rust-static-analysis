use std::fs;
use tree_sitter::Parser;

fn main() {
    // creating the new parser and setting the language to python
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Error loading Python grammar");

    // going to want to loop through the files in a directory
    // also going to want to make a final data struture to hand
    // off at the end. It will be mutable.
    analyze_file("test.py", &mut parser);
}

fn analyze_file(file_path: &str, parser: &mut Parser) {
    // defining the code that we want to parse
    let source = fs::read_to_string(file_path).expect("something went wrong with reading the file");

    // parsing the source code we are just borrowing this value so that we can use it
    // later on within the code
    let tree = parser.parse(&source, None);

    // printing out the results
    println!("{}", source);
    println!("{:?}", tree);
}
