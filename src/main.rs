use clap::Parser;
use mobile_ai_harness::cli;

fn main() {
    let parser = cli::MobileAiHarnessCli::parse();
    println!("Path: {:?}", parser.workspace_root);
    println!("Relative: {:?}", parser.workspace_root.is_relative());
    println!("Absolute: {:?}", parser.workspace_root.is_absolute());
    println!("Task: {:?}", parser.task);
    println!("Retries: {:?}", parser.no_of_retries);
    println!("Connection Timeout: {:?}", parser.connection_timeout);
    println!("Response Timeout: {:?}", parser.response_timeout);
    match parser.validate() {
        Ok(()) => {
            println!("Validated Successfully.");
        }
        Err(message) => {
            println!("{:?}", message);
        }
    }
}

#[test]
fn test_mobile_ai_harness_cli() {
    use std::path::PathBuf;
    let args = vec!["mobile-ai-harness", "-w", ".", "-t", "Hi"];
    let parser = cli::MobileAiHarnessCli::parse_from(args);
    assert_eq!(parser.task, "Hi");
    assert_eq!(parser.workspace_root, PathBuf::from("."));
}
