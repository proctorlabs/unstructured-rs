use super::*;

const TEST_JSON: &str = r#"{
    "some": "val"
}"#;

const TEST_YAML: &str = r#"
---
- this
- is
- an
- array
"#;

#[test]
fn basic_object() {
    let val: Value = serde_json::from_str(TEST_JSON).unwrap();
    println!("{:?}", val);
}

#[test]
fn basic_array() {
    let val: Value = serde_yaml::from_str(TEST_YAML).unwrap();
    println!("{:?}", val);
    let lis: Vec<String> = val.try_into().unwrap();
    println!("{:?}", lis);
}
