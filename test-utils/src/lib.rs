use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    time::SystemTime,
};
use tempfile::NamedTempFile;

pub struct CompiledBinary {
    compiled_binary: PathBuf,
    test_name: &'static str,
}

impl CompiledBinary {
    pub fn new(test_name: &'static str, source_code: &str) -> Self {
        let mut source_file = NamedTempFile::new().expect("failed to create source file");
        write!(source_file, "{}", source_code).expect("failed to write to source file");

        // compile bf to asm
        assert!(Command::new("target/debug/nerve")
            .arg(format!("{}", source_file.path().to_string_lossy()))
            .status()
            .expect("Failed to execute process")
            .success());

        let assembly_file = source_file.path().with_extension("s");
        let object_file = assembly_file.with_extension("o");
        // Using a non-standard ".bin" extension for the compiled binary here
        // because the tempfile has no extension. There isn't an easy way to
        // create the temp file with an extension, and if the compiled binary
        // overwrites the temp file, then it is deleted when the temp file goes
        // out of scope.
        let compiled_binary = object_file.clone().with_extension("bin");

        // compile asm into object file
        assert!(Command::new("nasm")
            .args(&[
                &assembly_file.to_string_lossy(),
                "-f",
                "elf64",
                "-o",
                &object_file.to_string_lossy(),
            ])
            .status()
            .expect("Failed to execute process")
            .success());

        // link asm object file into host binary
        assert!(Command::new("ld")
            .args(&[
                &object_file.to_string_lossy(),
                "-o",
                &compiled_binary.to_string_lossy()
            ])
            .status()
            .expect("Failed to execute process")
            .success());

        Self {
            compiled_binary,
            test_name,
        }
    }

    fn run(&self, input: &str) -> Output {
        let mut child = Command::new(&self.compiled_binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| {
                panic!(format!(
                    "Failed to spawn process {}",
                    &self.compiled_binary.to_string_lossy()
                ))
            });

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");

        child.wait_with_output().expect("Failed to read stdout")
    }

    pub fn test(&self, input: &str, expected_output: &str) {
        let start = SystemTime::now();
        let output = self.run(input);
        let elapsed = start.elapsed().expect("system time error");

        println!("{} completed in {:?}", &self.test_name, elapsed);

        // Trim end because we don't want to be picky about trailing newline
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim_end(), expected_output.trim_end());
    }
}
