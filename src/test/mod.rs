use super::*;

const PARSE_MAP_STR: &[&str] = &[
    r#"{
    "some": "val"
}"#,
    r#"
some: val
"#,
];

fn parse(index: usize, s: &str) -> Value {
    match index {
        0 => serde_json::from_str(s).unwrap(),
        _ => serde_yaml::from_str(s).unwrap(),
    }
}

#[rstest_parametrize(index, case(0), case(1))]
fn basic_object(index: usize) {
    let val: Value = parse(index, PARSE_MAP_STR[index]);
    println!("{:?}", val);
}
