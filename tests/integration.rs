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
