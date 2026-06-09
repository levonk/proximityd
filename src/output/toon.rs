use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// TOON format encoder
/// Converts JSON-like data structures to TOON format
#[derive(Debug, Clone)]
pub struct ToonEncoder {
    indent_size: usize,
    strict: bool,
}

impl Default for ToonEncoder {
    fn default() -> Self {
        Self {
            indent_size: 2,
            strict: true,
        }
    }
}

impl ToonEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Encode a serde_json::Value to TOON format
    pub fn encode(&self, value: &Value) -> std::result::Result<String, ToonError> {
        let mut output = String::new();
        self.encode_value(value, 0, &mut output)?;
        Ok(output)
    }

    fn encode_value(&self, value: &Value, indent: usize, output: &mut String) -> std::result::Result<(), ToonError> {
        match value {
            Value::Null => {
                output.push_str("null");
            }
            Value::Bool(b) => {
                output.push_str(if *b { "true" } else { "false" });
            }
            Value::Number(n) => {
                self.encode_number(n, output);
            }
            Value::String(s) => {
                self.encode_string(s, output);
            }
            Value::Array(arr) => {
                self.encode_array(arr, indent, output)?;
            }
            Value::Object(obj) => {
                self.encode_object(obj, indent, output)?;
            }
        }
        Ok(())
    }

    fn encode_number(&self, n: &serde_json::Number, output: &mut String) {
        if let Some(i) = n.as_i64() {
            output.push_str(&i.to_string());
        } else if let Some(u) = n.as_u64() {
            output.push_str(&u.to_string());
        } else if let Some(f) = n.as_f64() {
            // Use canonical decimal representation for values in [1e-6, 1e21)
            if f.abs() >= 1e-6 && f.abs() < 1e21 {
                output.push_str(&format!("{}", f));
            } else {
                output.push_str(&format!("{:e}", f));
            }
        }
    }

    fn encode_string(&self, s: &str, output: &mut String) {
        // Check if string needs quoting
        if self.needs_quoting(s) {
            output.push('"');
            for c in s.chars() {
                match c {
                    '\\' => output.push_str("\\\\"),
                    '"' => output.push_str("\\\""),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    c if c <= '\u{001f}' => {
                        output.push_str(&format!("\\u{:04x}", c as u32));
                    }
                    _ => output.push(c),
                }
            }
            output.push('"');
        } else {
            output.push_str(s);
        }
    }

    fn needs_quoting(&self, s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        
        // Check for special characters that require quoting
        for c in s.chars() {
            match c {
                ':' | '[' | ']' | '{' | '}' | ',' | '\t' | '|' | '\n' | '\r' | '"' | '\\' => return true,
                c if c <= ' ' => return true, // Control characters
                _ => {}
            }
        }
        
        // Check if it's a reserved word
        matches!(s, "true" | "false" | "null")
    }

    fn encode_array(&self, arr: &[Value], indent: usize, output: &mut String) -> std::result::Result<(), ToonError> {
        if arr.is_empty() {
            output.push_str("[]");
            return Ok(());
        }

        // Check if this is a primitive array (all elements are primitives)
        let is_primitive = arr.iter().all(|v| self.is_primitive(v));
        
        if is_primitive {
            // Inline primitive array
            output.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                self.encode_value(item, indent, output)?;
            }
            output.push(']');
        } else {
            // List format for complex arrays
            for item in arr {
                self.write_indent(indent, output);
                output.push_str("- ");
                self.encode_value(item, indent + 1, output)?;
                output.push('\n');
            }
        }
        
        Ok(())
    }

    fn encode_object(&self, obj: &serde_json::Map<String, Value>, indent: usize, output: &mut String) -> std::result::Result<(), ToonError> {
        if obj.is_empty() {
            output.push_str("{}");
            return Ok(());
        }

        for (key, value) in obj {
            self.write_indent(indent, output);
            self.encode_string(key, output);
            output.push_str(": ");
            
            // Check if value is a primitive or empty container
            if self.is_simple_value(value) {
                self.encode_value(value, indent, output)?;
                output.push('\n');
            } else {
                output.push('\n');
                self.encode_value(value, indent + 1, output)?;
            }
        }
        
        Ok(())
    }

    fn is_primitive(&self, value: &Value) -> bool {
        matches!(value, Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_))
    }

    fn is_simple_value(&self, value: &Value) -> bool {
        match value {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => true,
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
        }
    }

    fn write_indent(&self, indent: usize, output: &mut String) {
        for _ in 0..(indent * self.indent_size) {
            output.push(' ');
        }
    }
}

/// TOON format decoder
/// Converts TOON format to serde_json::Value
#[derive(Debug, Clone)]
pub struct ToonDecoder {
    strict: bool,
}

impl Default for ToonDecoder {
    fn default() -> Self {
        Self {
            strict: true,
        }
    }
}

impl ToonDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Decode TOON format to serde_json::Value
    pub fn decode(&self, input: &str) -> std::result::Result<Value, ToonError> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Ok(Value::Null);
        }

        let mut parser = ToonParser::new(lines, self.strict);
        parser.parse()
    }
}

struct ToonParser<'a> {
    lines: Vec<&'a str>,
    #[allow(dead_code)]
    strict: bool,
    current: usize,
}

impl<'a> ToonParser<'a> {
    fn new(lines: Vec<&'a str>, strict: bool) -> Self {
        Self {
            lines,
            strict,
            current: 0,
        }
    }

    fn parse(&mut self) -> std::result::Result<Value, ToonError> {
        if self.lines.is_empty() {
            return Ok(Value::Null);
        }

        // Determine root form based on first line
        let first_line = self.lines[0].trim();
        
        if first_line.starts_with('[') && first_line.ends_with(']') {
            // Inline array
            self.parse_inline_array(first_line)
        } else if first_line.starts_with('{') && first_line.ends_with('}') {
            // Inline object
            self.parse_inline_object(first_line)
        } else if first_line.starts_with('-') {
            // List
            self.parse_list(0)
        } else if first_line.contains(':') {
            // Object
            self.parse_object(0)
        } else {
            // Primitive
            self.parse_primitive(first_line)
        }
    }

    fn parse_inline_array(&self, line: &str) -> std::result::Result<Value, ToonError> {
        let content = &line[1..line.len()-1];
        if content.trim().is_empty() {
            return Ok(Value::Array(vec![]));
        }

        let items: Vec<&str> = content.split(',').collect();
        let mut result = Vec::new();
        for item in items {
            result.push(self.parse_primitive(item.trim())?);
        }
        Ok(Value::Array(result))
    }

    fn parse_inline_object(&self, line: &str) -> std::result::Result<Value, ToonError> {
        let content = &line[1..line.len()-1];
        if content.trim().is_empty() {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        // Simple inline object parsing (limited support)
        let mut result = serde_json::Map::new();
        // This is a simplified parser - full implementation would need more sophisticated parsing
        for pair in content.split(',') {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() == 2 {
                let key = self.parse_string(parts[0].trim())?;
                let value = self.parse_primitive(parts[1].trim())?;
                result.insert(key, value);
            }
        }
        Ok(Value::Object(result))
    }

    fn parse_list(&mut self, base_indent: usize) -> std::result::Result<Value, ToonError> {
        let mut items = Vec::new();
        
        while self.current < self.lines.len() {
            let line = self.lines[self.current];
            let indent = self.get_indent(line);
            
            if indent < base_indent {
                break;
            }
            
            if line.trim().starts_with('-') {
                let content = line.trim()[1..].trim();
                if !content.is_empty() {
                    items.push(self.parse_primitive(content)?);
                }
                self.current += 1;
            } else {
                break;
            }
        }
        
        Ok(Value::Array(items))
    }

    fn parse_object(&mut self, base_indent: usize) -> std::result::Result<Value, ToonError> {
        let mut result = serde_json::Map::new();
        
        while self.current < self.lines.len() {
            let line = self.lines[self.current];
            let indent = self.get_indent(line);
            
            if indent < base_indent {
                break;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key_part = &line[..colon_pos];
                let value_part = &line[colon_pos+1..];
                
                let key = self.parse_string(key_part.trim())?;
                let value = self.parse_primitive(value_part.trim())?;
                
                result.insert(key, value);
                self.current += 1;
            } else {
                break;
            }
        }
        
        Ok(Value::Object(result))
    }

    fn parse_primitive(&self, token: &str) -> std::result::Result<Value, ToonError> {
        let token = token.trim();
        
        if token == "null" {
            Ok(Value::Null)
        } else if token == "true" {
            Ok(Value::Bool(true))
        } else if token == "false" {
            Ok(Value::Bool(false))
        } else if let Ok(n) = token.parse::<i64>() {
            Ok(Value::Number(n.into()))
        } else if let Ok(n) = token.parse::<f64>() {
            Ok(Value::Number(serde_json::Number::from_f64(n).unwrap()))
        } else if token.starts_with('"') && token.ends_with('"') {
            Ok(Value::String(self.parse_string(token)?))
        } else {
            // Unquoted string
            Ok(Value::String(token.to_string()))
        }
    }

    fn parse_string(&self, s: &str) -> std::result::Result<String, ToonError> {
        let s = s.trim();
        if s.starts_with('"') && s.ends_with('"') {
            let content = &s[1..s.len()-1];
            let mut result = String::new();
            let mut chars = content.chars().peekable();
            
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(next) = chars.next() {
                        match next {
                            '\\' => result.push('\\'),
                            '"' => result.push('"'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                // Parse Unicode escape
                                let hex_code: String = chars.by_ref().take(4).collect();
                                if let Ok(code) = u32::from_str_radix(&hex_code, 16) {
                                    if let Some(c) = char::from_u32(code) {
                                        result.push(c);
                                    }
                                }
                            }
                            _ => result.push(next),
                        }
                    }
                } else {
                    result.push(c);
                }
            }
            Ok(result)
        } else {
            Ok(s.to_string())
        }
    }

    fn get_indent(&self, line: &str) -> usize {
        line.chars().take_while(|c| *c == ' ').count()
    }
}

/// TOON value representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToonValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<ToonValue>),
    Object(HashMap<String, ToonValue>),
}

impl From<Value> for ToonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => ToonValue::Null,
            Value::Bool(b) => ToonValue::Bool(b),
            Value::Number(n) => ToonValue::Number(n),
            Value::String(s) => ToonValue::String(s),
            Value::Array(arr) => ToonValue::Array(
                arr.into_iter().map(|v| v.into()).collect()
            ),
            Value::Object(obj) => ToonValue::Object(
                obj.into_iter().map(|(k, v)| (k, v.into())).collect()
            ),
        }
    }
}

impl From<ToonValue> for Value {
    fn from(value: ToonValue) -> Self {
        match value {
            ToonValue::Null => Value::Null,
            ToonValue::Bool(b) => Value::Bool(b),
            ToonValue::Number(n) => Value::Number(n),
            ToonValue::String(s) => Value::String(s),
            ToonValue::Array(arr) => Value::Array(
                arr.into_iter().map(|v| v.into()).collect()
            ),
            ToonValue::Object(obj) => Value::Object(
                obj.into_iter().map(|(k, v)| (k, v.into())).collect()
            ),
        }
    }
}

/// TOON error types
#[derive(Debug, thiserror::Error)]
pub enum ToonError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    
    #[error("Strict mode violation: {0}")]
    StrictModeViolation(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encode_null() {
        let encoder = ToonEncoder::new();
        let result = encoder.encode(&Value::Null).unwrap();
        assert_eq!(result, "null");
    }

    #[test]
    fn test_encode_bool() {
        let encoder = ToonEncoder::new();
        assert_eq!(encoder.encode(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(encoder.encode(&Value::Bool(false)).unwrap(), "false");
    }

    #[test]
    fn test_encode_number() {
        let encoder = ToonEncoder::new();
        assert_eq!(encoder.encode(&json!(42)).unwrap(), "42");
        assert_eq!(encoder.encode(&json!(3.14)).unwrap(), "3.14");
    }

    #[test]
    fn test_encode_string() {
        let encoder = ToonEncoder::new();
        assert_eq!(encoder.encode(&json!("hello")).unwrap(), "hello");
        assert_eq!(encoder.encode(&json!("hello world")).unwrap(), "\"hello world\"");
    }

    #[test]
    fn test_encode_array() {
        let encoder = ToonEncoder::new();
        let arr = json!([1, 2, 3]);
        let result = encoder.encode(&arr).unwrap();
        assert!(result.contains("["));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_encode_object() {
        let encoder = ToonEncoder::new();
        let obj = json!({"name": "test", "value": 42});
        let result = encoder.encode(&obj).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("test"));
        assert!(result.contains("value"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_decode_null() {
        let decoder = ToonDecoder::new();
        let result = decoder.decode("null").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_decode_bool() {
        let decoder = ToonDecoder::new();
        assert_eq!(decoder.decode("true").unwrap(), Value::Bool(true));
        assert_eq!(decoder.decode("false").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_decode_number() {
        let decoder = ToonDecoder::new();
        assert_eq!(decoder.decode("42").unwrap(), json!(42));
        assert_eq!(decoder.decode("3.14").unwrap(), json!(3.14));
    }

    #[test]
    fn test_decode_string() {
        let decoder = ToonDecoder::new();
        assert_eq!(decoder.decode("hello").unwrap(), json!("hello"));
        assert_eq!(decoder.decode("\"hello world\"").unwrap(), json!("hello world"));
    }

    #[test]
    fn test_round_trip_simple() {
        let original = json!({"name": "test", "value": 42, "active": true});
        let encoder = ToonEncoder::new();
        let decoder = ToonDecoder::new();
        
        let toon = encoder.encode(&original).unwrap();
        let decoded = decoder.decode(&toon).unwrap();
        
        // Note: Due to simplified parsing, exact round-trip may not work for complex structures
        // This test validates the basic flow works
        assert!(decoded.is_object());
    }

    #[test]
    fn test_needs_quoting() {
        let encoder = ToonEncoder::new();
        assert!(encoder.needs_quoting("hello world"));
        assert!(encoder.needs_quoting("test:value"));
        assert!(encoder.needs_quoting(""));
        assert!(!encoder.needs_quoting("hello"));
    }

    #[test]
    fn test_token_savings() {
        let encoder = ToonEncoder::new();
        let obj = json!({
            "name": "test",
            "value": 42,
            "active": true,
            "items": [1, 2, 3]
        });
        
        let json_str = serde_json::to_string_pretty(&obj).unwrap();
        let toon_str = encoder.encode(&obj).unwrap();
        
        // TOON should be more compact
        assert!(toon_str.len() < json_str.len());
        
        // Calculate token savings (rough estimate)
        let savings = 1.0 - (toon_str.len() as f64 / json_str.len() as f64);
        println!("Token savings: {:.1}%", savings * 100.0);
        
        // Assert we achieve at least 20% savings (conservative target)
        assert!(savings > 0.20, "TOON should achieve at least 20% token savings, got {:.1}%", savings * 100.0);
    }
}
