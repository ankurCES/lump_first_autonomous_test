use std::process::Command;

fn dispatchd_bin() -> String {
    env!("CARGO_BIN_EXE_dispatchd").to_string()
}

fn run(args: &[&str], home: &str) -> (String, String, i32) {
    let output = Command::new(dispatchd_bin())
        .args(args)
        .env("DISPATCHD_HOME", home)
        .output()
        .expect("failed to run dispatchd");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

#[test]
fn test_add_list_cancel_roundtrip() {
    let dir = tempfile::TempDir::new().unwrap();
    let home = dir.path().to_str().unwrap();

    // Add two jobs
    let (out, _, code) = run(&["add", "first job", "--priority", "high"], home);
    assert_eq!(code, 0, "add first: {out}");

    let (out, _, code) = run(&["add", "second job"], home);
    assert_eq!(code, 0, "add second: {out}");

    // List open — should see both
    let (out, _, code) = run(&["list", "--status", "open"], home);
    assert_eq!(code, 0);
    assert!(out.contains("first job"), "list should contain 'first job': {out}");
    assert!(out.contains("second job"), "list should contain 'second job': {out}");

    // Cancel job 1
    let (_, _, code) = run(&["cancel", "1"], home);
    assert_eq!(code, 0);

    // List open — should only see job 2
    let (out, _, code) = run(&["list", "--status", "open"], home);
    assert_eq!(code, 0);
    assert!(!out.contains("first job"), "cancelled job should not appear: {out}");
    assert!(out.contains("second job"));
}
