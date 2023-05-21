pub use serde;
pub use serde_json;

/// all strings (besides ones that can be represented as enums)
/// should be represented by StrVal. This allows users to specify
/// values as simple string values `"something".into()`
/// as well as complex mappings for Fn::GetAtt, Ref, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StrVal {
    String(String),
    Val(serde_json::Value),
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
