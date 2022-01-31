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
    let source_tree = parser.parse(&source, None).unwrap();
    let source_root = source_tree.root_node();
    let mut source_cursor = source_root.walk();

    // looping through all of the items of the tree
    let mut cond = source_cursor.goto_first_child();
    while cond {
        // pulling out the node to it's own variable
        let node = source_cursor.node();

        // only want to look through the function definitions
        if node.kind() == "function_definition" {
            // the name of the function will always be child at index 1
            let name = &source[node.child(1).unwrap().byte_range()];
            println!("{}:", name);

            // walking through the block of the function
            let block = node.child(4).unwrap();
            let mut block_cursor = block.walk();
            let mut walk_cond = block_cursor.goto_first_child();

            while walk_cond {
                match block_cursor.node().kind() {
                    // handling the return_statement structure
                    "return_statement" => {
                        let call = block_cursor.node().child(1).unwrap();
                        if call.kind() == "call" {
                            let call_name = &source[call.child(0).unwrap().byte_range()];
                            println!("  return_call: {}", call_name);
                        }
                    }
                    // handlig the expression statement structure
                    "expression_statement" => {
                        let child = block_cursor.node().child(0).unwrap();
                        match child.kind() {
                            "call" => {
                                let call_name = &source[child.child(0).unwrap().byte_range()];
                                println!("  call:        {}", call_name);
                            }
                            "assignment" => {
                                if child.child(2).unwrap().kind() == "call" {
                                    let call_name = &source
                                        [child.child(2).unwrap().child(0).unwrap().byte_range()];
                                    println!("  assign_call: {}", call_name);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                // going to the next node
                walk_cond = block_cursor.goto_next_sibling()
            }
        }

        // going to the next node
        cond = source_cursor.goto_next_sibling();
    }
}
