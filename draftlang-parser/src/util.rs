pub fn unescape_string(escaped_str: &str) -> String {
    // Crate for unescaping strings.
    use serde_json::Value;

    // Parse JSON string as a JSON value
    let v: Value = serde_json::from_str(escaped_str).unwrap();

    // Extract the string from the JSON value
    match v {
        Value::String(s) => s,
        _ => unreachable!(), // We know it's a string, so this shouldn't happen
    }
}
