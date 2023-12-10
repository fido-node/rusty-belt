use std::{fmt::Debug, process::Command};

use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::CacheKey,
};

#[cfg(test)]
use mockall::automock;

use super::{CacheSnapshot, Context, Model};

// Define a trait to represent the external command execution
#[cfg_attr(test, automock)]
pub trait CommandExecutor: Debug + Send + 'static {
    fn execute(&self, cmd: &str, current_directory: Option<String>) -> String;
}

#[derive(Debug, Clone)]
pub struct DefaultExecutor {}

impl DefaultExecutor {
    pub fn default() -> Box<dyn CommandExecutor> {
        Box::new(DefaultExecutor {})
    }
}
// Implement the trait for the actual Command struct

impl CommandExecutor for DefaultExecutor {
    fn execute(&self, cmd: &str, current_directory: Option<String>) -> String {
        let command = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(current_directory.unwrap_or(env!("HOME").to_string()))
            .output()
            .expect("failed to execute process");

        let str = String::from_utf8(command.stdout).expect("failed to convert stdout to String");
        str.trim().to_string()
    }
}

// Define a mock for the CommandExecutor trait
#[derive(Debug)]
pub struct Shell {
    pub cmd: String,
    pub cmd_result: Option<String>,
    pub use_pwd: bool,
    pub cmd_executor: Box<dyn CommandExecutor>,
}

impl Shell {
    pub fn new(cmd: String, use_pwd: bool, cmd_executor: Box<dyn CommandExecutor>) -> Self {
        Shell {
            cmd,
            cmd_result: None,
            use_pwd,
            cmd_executor,
        }
    }
}

impl Model for Shell {
    fn get_cache_key(&self) -> Option<CacheKey> {
        None
    }

    fn get_state(&self, context: &Context) -> segment_value::Segment {
        let cmd_result: Option<String> = if self.use_pwd {
            Some(
                self.cmd_executor
                    .execute(&self.cmd, context.current_directory.clone()),
            )
        } else {
            self.cmd_result.clone()
        };

        let mut result = belt::ShellExecutionResult::default();
        result.stdout = cmd_result.unwrap_or("".to_string());
        segment_value::Segment::ShellResult(result)
    }

    fn rehydrate(&mut self, _cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if !self.use_pwd {
            self.cmd_result = Some(self.cmd_executor.execute(&self.cmd, None));
        }

        Ok(())
    }
}

#[cfg(test)]
mod shell_tests {
    use mockall::predicate::eq;

    use crate::model::tests::generate_sample_context;

    use super::*;

    #[test]
    fn test_shell_get_state_use_pwd() {
        // Arrange
        let cmd = "echo hello".to_string();
        let use_pwd = true;
        let cmd_executor = Box::new(MockCommandExecutor::default());
        let context = generate_sample_context();
        let mut shell_model = Shell::new(cmd.clone(), use_pwd, cmd_executor);

        // Mock the behavior of the command executor
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .times(1)
            .with(eq(cmd), eq(Some("/path/to/current_directory".to_string())))
            .returning(|_, _| "hello".to_string());

        shell_model.cmd_executor = Box::new(mock_executor);

        // Act
        let result = shell_model.get_state(&context);

        // Assert
        assert_eq!(
            result,
            segment_value::Segment::ShellResult(belt::ShellExecutionResult {
                stdout: "hello".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_shell_rehydrate() {
        // Arrange
        let cmd = "echo hello".to_string();
        let use_pwd = false;
        let cmd_executor = Box::new(MockCommandExecutor::default());
        let mut shell_model = Shell::new(cmd.clone(), use_pwd, cmd_executor);

        // Mock the behavior of the command executor
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .times(1)
            .with(eq(cmd), eq(None::<String>))
            .returning(|_, _| "hello".to_string());

        shell_model.cmd_executor = Box::new(mock_executor);

        // Act
        let result = shell_model.rehydrate(&CacheSnapshot::default());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(shell_model.cmd_result, Some("hello".to_string()));
    }
}
