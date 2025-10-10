#[cfg(windows)]
const FAKELAKE_COMMAND_NAME: &str = "fakelake.exe";
#[cfg(not(windows))]
const FAKELAKE_COMMAND_NAME: &str = "fakelake";

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use ctor::{ctor, dtor};
    use predicates::prelude::*;
    use std::fs;
    use std::process::Command;

    use std::path::Path;

    use crate::FAKELAKE_COMMAND_NAME;

    #[ctor]
    fn init() {
        fs::create_dir_all("target/test_generated").ok();
    }

    #[dtor]
    fn shutdown() {
        fs::remove_dir_all("target/test_generated").ok();
        fs::remove_file("output.csv").ok();
        fs::remove_file("output.json").ok();
        fs::remove_file("target/csv_deterministic_test.csv").ok();
        fs::remove_file("target/csv_deterministic_test_2.csv").ok();
        fs::remove_file("target/csv_no_seed_test.csv").ok();
        fs::remove_file("target/csv_no_seed_test_2.csv").ok();
        fs::remove_file("target/csv_no_seed_test.yaml").ok();
    }

    #[test]
    fn given_no_args_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains(format!(
                "Usage: {} [OPTIONS] <COMMAND>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_help_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Usage: {} [OPTIONS] <COMMAND>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_generate_without_file_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .assert()
            .failure()
            .stderr(predicate::str::contains(format!(
                "Usage: {} generate <PATH_TO_CONFIG>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_existing_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_existing_should_fail(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("this/is/not_a_file.yaml"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_yaml_should_fail() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("src/main.rs"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_existing_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_with_one_not_existing_should_fail(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .arg(Path::new("this/is/not_a_file.yaml"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_one_parquet_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_one_csv_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_csv.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_one_json_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_json.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_same_seed_should_generate_identical_output() -> Result<(), Box<dyn std::error::Error>>
    {
        let config_path = Path::new("tests/csv_all_options.yaml");
        let output_path_1 = Path::new("target/csv_all_options.csv");
        let output_path_2 = Path::new("target/csv_all_options_2.csv");

        // First generation
        let mut cmd1 = Command::cargo_bin("fakelake")?;
        cmd1.arg("generate").arg(config_path).assert().success();

        // Verify first file was created and read its content
        assert!(output_path_1.exists(), "First output file was not created");
        let content1 = fs::read_to_string(output_path_1)?;

        // Rename first file to avoid conflict
        fs::rename(output_path_1, output_path_2)?;

        // Second generation
        let mut cmd2 = Command::cargo_bin("fakelake")?;
        cmd2.arg("generate").arg(config_path).assert().success();

        // Verify second file was created and read its content
        assert!(output_path_1.exists(), "Second output file was not created");
        let content2 = fs::read_to_string(output_path_1)?;

        // Compare the two files - they should be identical
        assert_eq!(
            content1, content2,
            "Files with same seed should be identical"
        );

        // Clean up
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();

        Ok(())
    }

    #[test]
    fn given_no_seed_should_generate_different_output() -> Result<(), Box<dyn std::error::Error>> {
        // Create a test config without seed
        let config_content = r#"
columns:
  - name: id
    provider: Increment.integer
  - name: random_score
    provider: Random.Number.i32
    min: 1
    max: 1000
  - name: random_bool
    provider: Random.bool

info:
  output_name: target/csv_no_seed_test
  output_format: csv
  rows: 50
"#;

        let config_path = Path::new("target/csv_no_seed_test.yaml");
        let output_path_1 = Path::new("target/csv_no_seed_test.csv");
        let output_path_2 = Path::new("target/csv_no_seed_test_2.csv");

        // Create config file
        fs::write(config_path, config_content)?;

        // Clean up any existing files
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();

        // First generation
        let mut cmd1 = Command::cargo_bin("fakelake")?;
        cmd1.arg("generate").arg(config_path).assert().success();

        // Verify first file was created and read its content
        assert!(output_path_1.exists(), "First output file was not created");
        let content1 = fs::read_to_string(output_path_1)?;

        // Rename first file to avoid conflict
        fs::rename(output_path_1, output_path_2)?;

        // Second generation
        let mut cmd2 = Command::cargo_bin("fakelake")?;
        cmd2.arg("generate").arg(config_path).assert().success();

        // Verify second file was created and read its content
        assert!(output_path_1.exists(), "Second output file was not created");
        let content2 = fs::read_to_string(output_path_1)?;

        // Compare the two files - they should be different (with high probability)
        // We check that at least some lines are different (excluding the header)
        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        assert_eq!(
            lines1.len(),
            lines2.len(),
            "Files should have same number of lines"
        );
        assert!(lines1.len() > 1, "Should have more than just header");

        // Header should be identical
        assert_eq!(lines1[0], lines2[0], "Headers should be identical");

        // At least one data line should be different
        let different_lines = lines1
            .iter()
            .zip(lines2.iter())
            .skip(1) // Skip header
            .any(|(line1, line2)| line1 != line2);

        assert!(
            different_lines,
            "At least some data lines should be different when no seed is provided"
        );

        // Clean up
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();
        fs::remove_file(config_path).ok();

        Ok(())
    }
}
