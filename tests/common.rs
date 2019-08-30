use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use tempfile::NamedTempFile;

pub struct CompiledBinary {
    compiled_binary: PathBuf,
}

impl CompiledBinary {
    pub fn new(source_code: &str) -> Self {
        let mut source_file = NamedTempFile::new().expect("failed to create source file");
        write!(source_file, "{}", source_code).expect("failed to write to source file");

        // compile bf to asm
        assert!(
            Command::new("target/debug/nerve")
                .arg(format!("{}", source_file.path().to_string_lossy()))
                .status()
                .expect("Failed to execute process")
                .success()
        );

        let assembly_file = source_file.path().clone().with_extension("s");
        let object_file = assembly_file.clone().with_extension("o");
        let compiled_binary = object_file.clone().with_extension("");

        // compile asm into object file
        assert!(
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                        "nasm {} -f elf64 -o {}", 
                        assembly_file.to_string_lossy(),
                        object_file.to_string_lossy(),
                    ))
                .status()
                .expect("Failed to execute process")
                .success()
        );

        // link asm object file into host binary
        assert!(
            Command::new("ld")
                .args(&[&object_file.to_string_lossy(), "-o", &compiled_binary.to_string_lossy()])
                .status()
                .expect("Failed to execute process")
                .success()
        );

        // TODO determine why this is needed
        // It seems the compiled binary is deleted at the same time as the temp file
        // unless the temp file is marked to be kept
        source_file.keep().expect("Failed to mark file for keeps");

        Self {
            compiled_binary,
        }
    }

    pub fn test(&self, input: &str, expected_output: &str) {
        let mut child = Command::new(&self.compiled_binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect(
                &format!("Failed to spawn process {}", &self.compiled_binary.to_string_lossy())
            );

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");

        let output = child.wait_with_output().expect("Failed to read stdout");
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
    }
}
