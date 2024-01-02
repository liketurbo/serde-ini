extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};

const TEST_1: &str = r"[child_1]
a=hello
b=world
";

#[derive(Serialize, Deserialize, Debug)]
struct Test1 {
    child_1: TestChild1,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestChild1 {
    a: String,
    b: String,
    c: Option<String>,
}

#[test]
fn none_value() {
    let des = serde_ini::from_str::<Test1>(TEST_1);
    assert!(des.is_ok());
    let ser = serde_ini::to_string(&des.unwrap()).unwrap();
    assert_eq!(ser, TEST_1);
}

#[derive(Serialize, Deserialize, Debug)]
struct Test2 {
    child_1: Option<TestChild2>,
    child_2: Option<TestChild2>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestChild2 {
    a: String,
    b: String,
}

#[test]
fn none_section() {
    let des = serde_ini::from_str::<Test2>(TEST_1);
    assert!(des.is_ok());
    let ser = serde_ini::to_string(&des.unwrap()).unwrap();
    println!("{}", ser);
    assert_eq!(ser, TEST_1);
}
