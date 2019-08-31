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
    // source code from http://www.hevanet.com/cristofd/brainfuck/short.b
    let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>->+>>+[<]<-]>>.>>---.+++++++..+++.>.<<-.>.+++.------.--------.>+.>++.";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    let expected_output = "Hello World!\n";
    compiled_binary.test(input, expected_output);
}

#[test]
fn reverse_input() {
    // source code from http://www.hevanet.com/cristofd/brainfuck/short.b
    let source_code = ">,[>,]<[.<]";
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "12345";
    let expected_output = "54321";
    compiled_binary.test(input, expected_output);
}
