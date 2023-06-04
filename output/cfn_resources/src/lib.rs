pub use serde;
pub use serde_json;

pub static mut FORCE_SERIALIZATION: bool = false;

pub fn wants_serialization<T>(t: &Option<T>) -> bool {
    unsafe {
        if FORCE_SERIALIZATION {
            return true;
        }
    }
    t.is_none()
}

/// all strings (besides ones that can be represented as enums)
/// should be represented by StrVal. This allows users to specify
/// values as simple string values `"something".into()`
/// as well as complex mappings for Fn::GetAtt, Ref, etc.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StrVal {
    String(String),
    Val(serde_json::Value),
}

impl Default for StrVal {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

pub trait ToOptStrVal {
    fn to_str_val(&self) -> Option<StrVal>;
}

impl From<&str> for StrVal {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<String> for StrVal {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<serde_json::Value> for StrVal {
    fn from(value: serde_json::Value) -> Self {
        Self::Val(value)
    }
}
impl ToOptStrVal for &str {
    fn to_str_val(&self) -> Option<StrVal> {
        Some(StrVal::String(self.to_string()))
    }
}
impl ToOptStrVal for String {
    fn to_str_val(&self) -> Option<StrVal> {
        Some(StrVal::String(self.to_string()))
    }
}

pub fn get_att(logical_name: &str, att_name: &str) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    let v = vec![
        serde_json::Value::String(logical_name.to_string()),
        serde_json::Value::String(att_name.to_string()),
    ];
    map.insert("Fn::GetAtt".to_string(), serde_json::Value::Array(v));
    serde_json::Value::Object(map)
}


pub fn get_ref(logical_name: &str) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("Ref".to_string(), serde_json::Value::String(logical_name.to_string()));
    serde_json::Value::Object(map)
}

/// statements are a tuple of:
/// - effect (must be Allow or Deny)
/// - action
/// - resource
/// - principal (omitted if empty)
pub fn create_policy_doc(statements: &[(String, String, StrVal, StrVal)]) -> serde_json::Value {
    let mut map = serde_json::Map::default();
    map.insert("Version".to_string(), serde_json::Value::String("2012-10-17".to_string()));
    let mut statements_out = vec![];
    for (effect, action, resource, principal) in statements {
        let mut statement_obj = serde_json::Map::default();
        statement_obj.insert("Effect".to_string(), serde_json::Value::String(effect.to_string()));
        statement_obj.insert("Action".to_string(), serde_json::Value::String(action.to_string()));
        match resource {
            StrVal::String(s) => {
                statement_obj.insert("Resource".to_string(), serde_json::Value::String(s.to_string()));
            }
            StrVal::Val(v) => {
                statement_obj.insert("Resource".to_string(), v.clone());
            }
        }
        match principal {
            StrVal::String(s) => {
                if !s.is_empty() {
                    statement_obj.insert("Principal".to_string(), serde_json::Value::String(s.to_string()));
                }
            }
            StrVal::Val(v) => {
                statement_obj.insert("Principal".to_string(), v.clone());
            }
        }
        statements_out.push(serde_json::Value::Object(statement_obj));
    }
    map.insert("Statement".to_string(), serde_json::Value::Array(statements_out));
    serde_json::Value::Object(map)
}

/// creates an object like:
/// { "Fn::Select" : [ "2", { "Fn::Split": ["/", {...}] } ] }
pub fn select_split(index: usize, delimiter: &str, stuff_to_split: serde_json::Value) -> serde_json::Value {
    // form the split map
    let mut split_map = serde_json::Map::new();
    let split_map_v = vec![
        serde_json::Value::String(delimiter.to_string()),
        stuff_to_split,
    ];
    split_map.insert("Fn::Split".to_string(), serde_json::Value::Array(split_map_v));
    // form the select map:
    let mut map = serde_json::Map::new();
    let v = vec![
        serde_json::Value::String(index.to_string()),
        serde_json::Value::Object(split_map),
    ];
    map.insert("Fn::Select".to_string(), serde_json::Value::Array(v));
    serde_json::Value::Object(map)
}

pub trait CfnResource {
    /// returns a string like 'AWS::CloudFront::Distribution'
    fn type_string(&self) -> &'static str;

    fn properties(&self) -> serde_json::Value;

    /// returns Err(string) if there is a validation error.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// validate first by the default validation function,
    /// and then add optional extra check via passing your own external validation function.
    fn validate_extern(&self, cb: fn(&Self) -> Result<(), String> ) -> Result<(), String>
        where Self: Sized
    {
        self.validate()?;
        cb(self)
    }

    /// like validate_extern, but does not use the default validation. This method
    /// only validates via your custom validation function.
    fn validate_override(&self, cb: fn(&Self) -> Result<(), String> ) -> Result<(), String>
        where Self: Sized
    {
        cb(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(serde::Serialize)]
    struct Example {
        #[serde(rename = "SourceSecurityGroupName")]
        pub source_security_group_name: StrVal,
    }

    #[test]
    fn serialization_works() {
        let simple = Example {
            source_security_group_name: StrVal::String("dsa".to_string()),
        };
        let out = serde_json::to_string(&simple).expect("Failed to serialize");
        assert_eq!(out, r#"{"SourceSecurityGroupName":"dsa"}"#);
        let mut map = serde_json::value::Map::new();
        let v = vec![
            serde_json::Value::String("myELB".to_string()),
            serde_json::Value::String("SourceSecurityGroup.GroupName".to_string()),
        ];
        map.insert("Fn::GetAtt".to_string(), serde_json::Value::Array(v));
        let obj = serde_json::Value::Object(map);
        let complex = Example {
            source_security_group_name: StrVal::Val(obj),
        };
        let out = serde_json::to_string(&complex).expect("Failed to serialize");
        assert_eq!(out, r#"{"SourceSecurityGroupName":{"Fn::GetAtt":["myELB","SourceSecurityGroup.GroupName"]}}"#);
    }
}
