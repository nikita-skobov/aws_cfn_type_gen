/// Specifies a prepared statement for use with SQL queries in Athena.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPreparedStatement {
    ///
    /// The description of the prepared statement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The query string for the prepared statement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 262144
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStatement")]
    pub query_statement: cfn_resources::StrVal,

    ///
    /// The name of the prepared statement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z_][a-zA-Z0-9_@:]{1,256}
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatementName")]
    pub statement_name: cfn_resources::StrVal,

    ///
    /// The workgroup to which the prepared statement belongs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkGroup")]
    pub work_group: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnPreparedStatement {
    fn type_string(&self) -> &'static str {
        "AWS::Athena::PreparedStatement"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.query_statement;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 262144 as _ {
                return Err(format!(
                    "Max validation failed on field 'query_statement'. {} is greater than 262144",
                    s.len()
                ));
            }
        }

        let the_val = &self.query_statement;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'query_statement'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.statement_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'statement_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.statement_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'statement_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
