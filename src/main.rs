use std::fs;
use tree_sitter::Parser;

// TODO probably going to actually want to use a hashmap for the final data structure
// since it will make searching through the values alot quicker than in a list;
// it will also probably help with the conversion of the map into JSON.

fn main() {
    // creating the new parser and setting the language to python
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Error loading Python grammar");

    // need to loop through file
    let mut data = analyze_file("test.py", &mut parser);

    // backfilling the called_by values now that we have the full data 
    backfill_called_by(&mut data);
}

fn backfill_called_by(data: &mut Vec<FuncDesc>) {
    println!("{:?}", data)
}


#[derive(Debug, PartialEq)]
struct FuncDesc {
    name: String,
    called_by: Vec<String>,
    called: Vec<String>,
}

fn analyze_file(file_path: &str, parser: &mut Parser) -> Vec<FuncDesc> {
    // initializing the response varible
    let mut resp: Vec<FuncDesc> = vec![];

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
            resp.push(analyze_function(node, &source));
        }

        // going to the next node
        cond = source_cursor.goto_next_sibling();
    }

    resp
}

// TODO eventually needs to return FuncDesc
fn analyze_function(node: tree_sitter::Node, source_string: &String) -> FuncDesc {
    // initializing the final data structure
    let mut resp: FuncDesc = FuncDesc {
        name: source_string[node.child(1).unwrap().byte_range()].to_string(),
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
                    resp.called.push(call_name.to_string());
                }
            }
            // handlig the expression statement structure
            "expression_statement" => {
                let child = block_cursor.node().child(0).unwrap();
                match child.kind() {
                    "call" => {
                        let call_name = &source_string[child.child(0).unwrap().byte_range()];
                        resp.called.push(call_name.to_string());
                    }
                    "assignment" => {
                        if child.child(2).unwrap().kind() == "call" {
                            let call_name = &source_string
                                [child.child(2).unwrap().child(0).unwrap().byte_range()];
                            resp.called.push(call_name.to_string());
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

    resp
}

#[test]
fn test_analyze_function() {
    // tests the analyze_function function used for pulling out meta data
    // about an input tree_sitter `function_definition` node.

    // initializing the parser
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Error loading Python grammar");

    // creating the source code used for testing
    let source = r#"
    def func1():
        """function definition 1"""
        function_print()
        a = 20
        b = 25
        x = addition(a, b)
        return func2()
    "#
    .to_string();

    // parsing the input
    let source_tree = parser.parse(&source, None).unwrap();
    let source_root = source_tree.root_node();
    let source_node = source_root.child(0).unwrap();

    // defining the expected output
    let expected: FuncDesc = FuncDesc {
        name: String::from("func1"),
        called_by: vec![],
        called: vec!["function_print", "addition", "func2"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    // checking that everything is working correctly
    assert_eq!(analyze_function(source_node, &source), expected);
}
