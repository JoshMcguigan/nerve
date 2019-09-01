use std::{
    env,
    fs::{canonicalize, read_dir, File},
    io::Write,
    path::Path,
};

fn main() {
    let outfile_path = Path::new(&env::var("OUT_DIR").unwrap()).join("gen_tests.rs");
    let mut outfile = File::create(outfile_path).unwrap();
    for dir in read_dir("bf-examples").unwrap() {
        let dir = dir.unwrap().path();
        let test_name = dir.file_name().unwrap().to_string_lossy();

        // paths need to be converted to absolute paths with canonicalize
        // so they are still correct when used from a file in the OUT_DIR
        let mut source_file = dir.clone();
        source_file.push(&format!("{}.b", test_name));
        let source_file = canonicalize(source_file).unwrap();

        let mut input_file = dir.clone();
        input_file.push(&format!("{}.in", test_name));
        let input_file = canonicalize(input_file).unwrap();

        let mut output_file = dir.clone();
        output_file.push(&format!("{}.out", test_name));
        let output_file = canonicalize(output_file).unwrap();

        write!(
            outfile,
            r#"
            #[test]
            fn {test_name}() {{
                let compiled_binary = test_utils::CompiledBinary::new("{test_name}", include_str!("{source_file}"));
                compiled_binary.test(include_str!("{input_file}"), include_str!("{output_file}"));
            }}
        "#,
            test_name = test_name,
            source_file = source_file.to_string_lossy(),
            input_file = input_file.to_string_lossy(),
            output_file = output_file.to_string_lossy(),
        )
        .unwrap();
    }
}
