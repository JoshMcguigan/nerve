mod common;
use common::CompiledBinary;

#[test]
fn output_a() {
    let source_code = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    let expected_output = "A";
    compiled_binary.test(input, expected_output);
}

#[test]
fn single_loop() {
    let source_code = "+[-]";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    let expected_output = "";
    compiled_binary.test(input, expected_output);
}

#[test]
fn multiple_loops() {
    let source_code = "+[-]>++[-]";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    let expected_output = "";
    compiled_binary.test(input, expected_output);
}

#[test]
fn hello_world() {
    let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>->+>>+[<]<-]>>.>>---.+++++++..+++.>.<<-.>.+++.------.--------.>+.>++.";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    let expected_output = "Hello World!\n";
    compiled_binary.test(input, expected_output);
}
