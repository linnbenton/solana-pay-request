use solana_pay_request::core::{handle_execute, RuntimeContext};

struct MockContext;

impl RuntimeContext for MockContext {
    fn read_config(&self, key: &str) -> String {
        if key == "allowed_mints" {
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()
        } else {
            "".to_string()
        }
    }
    fn log_record(&self, _level: &str, _msg: &str) {}
}

#[test]
fn test_execute_success() {
    let json_input = r#"{"recipient":"7xK9sW2d3fG4hJ5kL6mN7pQ8rS9tU0vW1xY2zA3bC4dE","amount":"25.00","label":"Cafe Test"}"#;
    let res = handle_execute(json_input, &MockContext);
    assert!(res.is_ok());
}

#[test]
fn test_execute_fail_closed_unauthorized_mint() {
    let json_input = r#"{"recipient":"7xK9sW2d3fG4hJ5kL6mN7pQ8rS9tU0vW1xY2zA3bC4dE","amount":"25.00","spl_mint":"MaliciousMintAddress1111","label":"Cafe Test"}"#;
    let res = handle_execute(json_input, &MockContext);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Execution Denied"));
}