#[cfg(test)]
mod tests {
    use clap::Parser;
    use mobile_ai_harness::cli::MobileAiHarnessCli;
    use std::fs::{self, File};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Scoped temp fixture that creates a unique directory and a dummy file,
    /// and cleans them up automatically on drop.
    struct TestDirFixture {
        dir_path: PathBuf,
        file_path: PathBuf,
    }

    impl TestDirFixture {
        fn new(label: &str) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let base = std::env::temp_dir().join(format!("harness_test_{}_{}", label, timestamp));
            fs::create_dir_all(&base).expect("Failed to create temporary test directory");

            let file_path = base.join("dummy_file.txt");
            File::create(&file_path).expect("Failed to create dummy file fixture");

            Self {
                dir_path: base,
                file_path,
            }
        }
    }

    impl Drop for TestDirFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir_path);
        }
    }

    // ========================================================================
    // 1. Argument Parsing & Default Values (Clap Layer)
    // ========================================================================

    #[test]
    fn test_parse_minimal_flags_applies_correct_defaults() {
        let fixture = TestDirFixture::new("defaults");
        let args = [
            "mobile-ai-harness",
            "-w",
            fixture.dir_path.to_str().unwrap(),
            "-t",
            "Locate auth handler",
        ];

        let cli = MobileAiHarnessCli::try_parse_from(args).expect("Should parse minimal flags");

        assert_eq!(cli.workspace_root, fixture.dir_path);
        assert_eq!(cli.task, "Locate auth handler");
        assert_eq!(cli.no_of_retries, 3, "Default retries must be 3");
        assert_eq!(
            cli.connection_timeout, 2,
            "Default connection timeout must be 2"
        );
        assert_eq!(
            cli.response_timeout, 90,
            "Default response timeout must be 90"
        );
    }

    #[test]
    fn test_parse_all_explicit_flags_overrides_defaults() {
        let fixture = TestDirFixture::new("explicit");
        let args = [
            "mobile-ai-harness",
            "--workspace-root",
            fixture.dir_path.to_str().unwrap(),
            "--task",
            "Index symbols",
            "--no-of-retries",
            "5",
            "--connection-timeout-secs",
            "10",
            "--response-timeout-secs",
            "45",
        ];

        let cli =
            MobileAiHarnessCli::try_parse_from(args).expect("Should parse explicit long flags");

        assert_eq!(cli.workspace_root, fixture.dir_path);
        assert_eq!(cli.task, "Index symbols");
        assert_eq!(cli.no_of_retries, 5);
        assert_eq!(cli.connection_timeout, 10);
        assert_eq!(cli.response_timeout, 45);
    }

    #[test]
    fn test_missing_workspace_flag_fails() {
        let args = ["mobile-ai-harness", "-t", "Some task"];
        let result = MobileAiHarnessCli::try_parse_from(args);
        assert!(
            result.is_err(),
            "Clap must fail when -w / --workspace-root is missing"
        );
    }

    #[test]
    fn test_missing_task_flag_fails() {
        let fixture = TestDirFixture::new("missing_task");
        let args = [
            "mobile-ai-harness",
            "-w",
            fixture.dir_path.to_str().unwrap(),
        ];
        let result = MobileAiHarnessCli::try_parse_from(args);
        assert!(
            result.is_err(),
            "Clap must fail when -t / --task is missing"
        );
    }

    #[test]
    fn test_u8_overflow_fails_parsing() {
        let fixture = TestDirFixture::new("u8_overflow");
        let args = [
            "mobile-ai-harness",
            "-w",
            fixture.dir_path.to_str().unwrap(),
            "-t",
            "Task",
            "-n",
            "300", // Exceeds u8::MAX (255)
        ];
        let result = MobileAiHarnessCli::try_parse_from(args);
        assert!(
            result.is_err(),
            "Clap must reject integer values greater than 255 for u8"
        );
    }

    // ========================================================================
    // 2. Domain Validation Invariants (validate() method)
    // ========================================================================

    #[test]
    fn test_validate_success_on_valid_inputs() {
        let fixture = TestDirFixture::new("valid_flow");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "Find all router definitions".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 90,
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_fails_when_workspace_does_not_exist() {
        let non_existent = std::env::temp_dir().join("missing_directory_12345_xyz");
        let cli = MobileAiHarnessCli {
            workspace_root: non_existent,
            task: "Some task".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Workspace does not exists"));
    }

    #[test]
    fn test_validate_fails_when_workspace_is_a_file() {
        let fixture = TestDirFixture::new("file_not_dir");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.file_path.clone(), // Points to regular file
            task: "Some task".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Workspace is not a directory"));
    }

    #[test]
    fn test_validate_fails_when_task_is_empty() {
        let fixture = TestDirFixture::new("empty_task");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Task is empty"));
    }

    #[test]
    fn test_validate_fails_when_task_is_pure_whitespace() {
        let fixture = TestDirFixture::new("whitespace_task");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "    \t \n  ".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Task is empty"));
    }

    #[test]
    fn test_validate_fails_when_no_of_retries_is_zero() {
        let fixture = TestDirFixture::new("zero_retries");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "Valid task".to_string(),
            no_of_retries: 0,
            connection_timeout: 2,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("No of retries cannot be zero"));
    }

    #[test]
    fn test_validate_fails_when_connection_timeout_is_zero() {
        let fixture = TestDirFixture::new("zero_conn_timeout");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "Valid task".to_string(),
            no_of_retries: 3,
            connection_timeout: 0,
            response_timeout: 90,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Connection timeout cannot be zero"));
    }

    #[test]
    fn test_validate_fails_when_response_timeout_is_zero() {
        let fixture = TestDirFixture::new("zero_resp_timeout");
        let cli = MobileAiHarnessCli {
            workspace_root: fixture.dir_path.clone(),
            task: "Valid task".to_string(),
            no_of_retries: 3,
            connection_timeout: 2,
            response_timeout: 0,
        };

        let err = cli.validate().unwrap_err();
        assert!(err.contains("Respone timeout cannot be zero"));
    }
}
