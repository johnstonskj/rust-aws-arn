use aws_arn::ARN;
use std::str::FromStr;

const EXAMPLES: &str = include_str!("examples.txt");

#[test]
fn test_examples_from_file() {
    for (line, arn_str) in EXAMPLES.lines().enumerate() {
        println!("{:0>4}: {}", line + 1, arn_str);
        let parsed = ARN::from_str(arn_str);
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }
}
