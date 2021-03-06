extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_in_valid_string() {
    let mut params = Map::new();
    params.assign("in", Value::String("1".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(),
               &Value::String("1".to_owned()));
}

#[test]
fn test_in_invalid_string() {
    let mut params = Map::new();
    params.assign("in", Value::String("2".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must be among the options: [\"1\", 2].".to_owned()]);
}

#[test]
fn test_in_valid_numeric() {
    let mut params = Map::new();
    params.assign("in", Value::U64(2)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(), &Value::U64(2));
}

#[test]
fn test_in_invalid_numeric() {
    let mut params = Map::new();
    params.assign("in", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must be among the options: [\"1\", 2].".to_owned()]);
}

#[test]
fn test_in_valid_empty() {
    let mut params = Map::new();
    params.assign("in", Value::String("".into())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(),
               &Value::String("".into()));
}

#[test]
fn test_in_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]), None);
}

#[test]
fn test_in_invalid_null() {
    let mut params = Map::new();
    params.assign("in", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in",
                 vec![Rule::In(vec![Value::String("1".into()), Value::U64(2)])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must be among the options: [\"1\", 2].".to_owned()]);
}
