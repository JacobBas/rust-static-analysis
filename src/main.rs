use std::collections::HashMap;
use std::fs;
use tree_sitter::Parser;

fn main() {
    // creating the new parser and setting the language to python
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Error loading Python grammar");

    // initializing the data
    let mut data: HashMap<String, FuncDesc> = HashMap::new();

    // need to loop through file
    analyze_file("test.py", &mut parser, &mut data);

    backfill_called_by(&mut data)
}

fn backfill_called_by(data: &mut HashMap<String, FuncDesc>) {
    // TODO need to work on backfilling the data now that I have it all in a hash map
    println!("{:?}", data)
}

fn analyze_file(file_path: &str, parser: &mut Parser, data: &mut HashMap<String, FuncDesc>) {
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
            analyze_function(node, &source, data);
        }

        // going to the next node
        cond = source_cursor.goto_next_sibling();
    }
}

#[derive(Debug, PartialEq)]
struct FuncDesc {
    called_by: Vec<String>,
    called: Vec<String>,
}

// TODO eventually needs to return FuncDesc
fn analyze_function(
    node: tree_sitter::Node,
    source_string: &String,
    data: &mut HashMap<String, FuncDesc>,
) {
    // initializing the description
    let mut desc: FuncDesc = FuncDesc {
        called_by: vec![],
        called: vec![],
    };

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
                    let call_name = &source_string[call.child(0).unwrap().byte_range()];
                    desc.called.push(call_name.to_string());
                }
            }
            // handlig the expression statement structure
            "expression_statement" => {
                let child = block_cursor.node().child(0).unwrap();
                match child.kind() {
                    "call" => {
                        let call_name = &source_string[child.child(0).unwrap().byte_range()];
                        desc.called.push(call_name.to_string());
                    }
                    "assignment" => {
                        if child.child(2).unwrap().kind() == "call" {
                            let call_name = &source_string
                                [child.child(2).unwrap().child(0).unwrap().byte_range()];
                            desc.called.push(call_name.to_string());
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

    // creating a new key for the added function
    data.insert(
        source_string[node.child(1).unwrap().byte_range()].to_string(),
        desc,
    );
}
