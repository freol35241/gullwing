use std::io::Write;
use std::process::{Command, Stdio};

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn run_shuffle(
        input_pattern: &str,
        output_pattern: &str,
        input_data: &str,
    ) -> Result<String, String> {
        // Build the example first
        let build = Command::new("cargo")
            .args(&["build", "--example", "shuffle"])
            .output()
            .map_err(|e| format!("Failed to build shuffle example: {}", e))?;

        if !build.status.success() {
            return Err(format!(
                "Build failed: {}",
                String::from_utf8_lossy(&build.stderr)
            ));
        }

        // Run the shuffle example
        let mut child = Command::new("target/debug/examples/shuffle")
            .arg(input_pattern)
            .arg(output_pattern)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn shuffle: {}", e))?;

        // Write input data
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input_data.as_bytes())
                .map_err(|e| format!("Failed to write to stdin: {}", e))?;
        }

        // Get output
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to wait for shuffle: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Shuffle failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[test]
    fn test_shuffle_simple_reorder() {
        let input = "{name} {age:d}";
        let output = "{age:d} {name}";
        let data = "Alice 30\nBob 25\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "30 Alice\n25 Bob\n");
    }

    #[test]
    fn test_shuffle_log_transformation() {
        let input = "{timestamp} {level} {message}";
        let output = "[{level}] {message}";
        let data = "2024-01-15T10:30:00 INFO Server started\n2024-01-15T10:30:01 ERROR Connection failed\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "[INFO] Server started\n[ERROR] Connection failed\n");
    }

    #[test]
    fn test_shuffle_csv_reformat() {
        let input = "{id:d},{name},{score:f}";
        let output = "ID: {id:03d} | Name: {name} | Score: {score:.1f}";
        let data = "5,Alice,95.7\n10,Bob,87.3\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(
            result,
            "ID: 005 | Name: Alice | Score: 95.7\nID: 010 | Name: Bob | Score: 87.3\n"
        );
    }

    #[test]
    fn test_shuffle_extract_fields() {
        let input = "{a} {b} {c}";
        let output = "{b}";
        let data = "first second third\nuno dos tres\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "second\ndos\n");
    }

    #[test]
    fn test_shuffle_integer_formatting() {
        let input = "{value:d}";
        let output = "{value:05d}";
        let data = "42\n123\n7\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "00042\n00123\n00007\n");
    }

    #[test]
    fn test_shuffle_hex_conversion() {
        let input = "{value:d}";
        let output = "0x{value:x}";
        let data = "255\n16\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "0xff\n0x10\n");
    }

    #[test]
    fn test_shuffle_no_match_lines_skipped() {
        let input = "{value:d}";
        let output = "{value}";
        let data = "123\nnot a number\n456\n";

        let result = run_shuffle(input, output, data).unwrap();
        // Lines that don't match should be skipped
        assert_eq!(result, "123\n456\n");
    }

    #[test]
    fn test_shuffle_empty_input() {
        let input = "{value}";
        let output = "{value}";
        let data = "";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_shuffle_multiple_fields_same_value() {
        let input = "{x}";
        let output = "{x} {x} {x}";
        let data = "test\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "test test test\n");
    }

    #[test]
    fn test_shuffle_float_precision() {
        let input = "{value:f}";
        let output = "{value:.2f}";
        let data = "3.14159\n2.71828\n";

        let result = run_shuffle(input, output, data).unwrap();
        assert_eq!(result, "3.14\n2.72\n");
    }
}
