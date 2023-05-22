pub use serde;
pub use serde_json;

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

pub fn get_att(logical_name: &str, att_name: &str) -> Option<StrVal> {
    let mut map = serde_json::Map::new();
    let v = vec![
        serde_json::Value::String(logical_name.to_string()),
        serde_json::Value::String(att_name.to_string()),
    ];
    map.insert("Fn::GetAtt".to_string(), serde_json::Value::Array(v));
    Some(StrVal::Val(serde_json::Value::Object(map)))
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
