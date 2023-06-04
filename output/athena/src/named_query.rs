/// The AWS::Athena::NamedQuery resource specifies an Amazon Athena saved query, where QueryString contains the SQL query statements that       make up the query.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNamedQuery {
    ///
    /// The database to which the query belongs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: cfn_resources::StrVal,

    ///
    /// The query description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The query name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The SQL statements that make up the query.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 262144
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueryString")]
    pub query_string: cfn_resources::StrVal,

    ///
    /// The name of the workgroup that contains the named query.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [a-zA-Z0-9._-]{1,128}
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub work_group: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_named_query_id: CfnNamedQuerynamedqueryid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNamedQuerynamedqueryid;
impl CfnNamedQuerynamedqueryid {
    pub fn att_name(&self) -> &'static str {
        r#"NamedQueryId"#
    }
}

impl cfn_resources::CfnResource for CfnNamedQuery {
    fn type_string(&self) -> &'static str {
        "AWS::Athena::NamedQuery"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'database'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.database;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'database'. {} is less than 1",
                    s.len()
                ));
            }
        }

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

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.query_string;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 262144 as _ {
                return Err(format!(
                    "Max validation failed on field 'query_string'. {} is greater than 262144",
                    s.len()
                ));
            }
        }

        let the_val = &self.query_string;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'query_string'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
