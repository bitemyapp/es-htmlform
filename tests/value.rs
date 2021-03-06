use es_htmlform::value::{ValueMap, Value, urldecode, UrlDecodingError};

#[test]
fn test_parse_urlencoded_one_key_one_val() {
    let values = ValueMap::from_urlencoded(b"foo=1").unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(
        values.get("foo").unwrap(), &vec![Value::new("1")]);
}

#[test]
fn test_parse_urlencoded_one_key_no_val() {
    let values = ValueMap::from_urlencoded(b"foo").unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(values.get("foo").unwrap(), &vec![]);
}

#[test]
fn test_parse_urlencoded_one_key_two_vals() {
    let values = ValueMap::from_urlencoded(b"foo=1&foo=2").unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(
        values.get("foo").unwrap(),
        &vec![Value::new("1"), Value::new("2")]);
}

#[test]
fn test_parse_urlencoded_two_keys() {
    let values = ValueMap::from_urlencoded(b"foo=1&bar=2").unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(
        values.get("foo").unwrap(), &vec![Value::new("1")]);
    assert_eq!(
        values.get("bar").unwrap(), &vec![Value::new("2")]);
}

#[test]
fn test_parse_urlencoded_encoded_correctly() {
    let values = ValueMap::from_urlencoded(b"foo=foo%20bar").unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(
        values.get("foo").unwrap(), &vec![Value::new("foo bar")]);
}

#[test]
fn test_parse_urlencoded_encoded_invalid_char() {
    assert_eq!(
        ValueMap::from_urlencoded(b"foo=foo%2xbar"),
        Err(UrlDecodingError::new("invalid encoding sequence")));
}

#[test]
fn test_parse_urldecode_plus() {
    assert_eq!(urldecode(b"foo+bar").unwrap(), "foo bar");
}

#[test]
fn test_deserialize() {
    let values = serde_json::from_str::<ValueMap>(r#"{"foo":"bar"}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("bar")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":["bar"]}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("bar")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":["bar","baz"]}"#).unwrap();
    assert_eq!(
        values.values("foo").unwrap(),
        &vec![Value::new("bar"), Value::new("baz")]);

    let values = serde_json::from_str::<ValueMap>(r#"{"foo":123}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("123")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":[123, 456]}"#).unwrap();
    assert_eq!(
        values.values("foo").unwrap(),
        &vec![Value::new("123"), Value::new("456")]);

    let values = serde_json::from_str::<ValueMap>(r#"{"foo":123.4}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("123.4")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":[123.4, 456.7]}"#).unwrap();
    assert_eq!(
        values.values("foo").unwrap(),
        &vec![Value::new("123.4"), Value::new("456.7")]);

    let values = serde_json::from_str::<ValueMap>(r#"{"foo":-123}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("-123")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":[-123, -456]}"#).unwrap();
    assert_eq!(
        values.values("foo").unwrap(),
        &vec![Value::new("-123"), Value::new("-456")]);

    let values = serde_json::from_str::<ValueMap>(
        r#"{"foo":true,"bar":false}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![Value::new("on")]);
    assert_eq!(values.values("bar").unwrap(), &vec![Value::new("off")]);

    let values = serde_json::from_str::<ValueMap>(r#"{"foo":null}"#).unwrap();
    assert_eq!(values.values("foo").unwrap(), &vec![]);
}
