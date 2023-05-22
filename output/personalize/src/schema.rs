/// Creates an Amazon Personalize schema from the specified schema string. The schema you create    must be in Avro JSON format.
///
/// Amazon Personalize recognizes three schema variants. Each schema is associated with a dataset    type and has a set of required field and keywords. If you are creating a schema for a dataset in a Domain dataset group, you   provide the domain of the Domain dataset group.   You specify a schema when you call CreateDataset.
///
/// For more information on schemas, see    Datasets and schemas.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSchema {
    ///
    /// The domain of a schema that you created for a dataset in a Domain dataset group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECOMMERCE | VIDEO_ON_DEMAND
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<SchemaDomainEnum>,

    ///
    /// The name of the schema.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The schema.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Schema")]
    pub schema: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_schema_arn: CfnSchemaschemaarn,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SchemaDomainEnum {
    /// ECOMMERCE
    #[serde(rename = "ECOMMERCE")]
    Ecommerce,

    /// VIDEO_ON_DEMAND
    #[serde(rename = "VIDEO_ON_DEMAND")]
    Videoondemand,
}

impl Default for SchemaDomainEnum {
    fn default() -> Self {
        SchemaDomainEnum::Ecommerce
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSchemaschemaarn;
impl CfnSchemaschemaarn {
    pub fn att_name(&self) -> &'static str {
        r#"SchemaArn"#
    }
}

impl cfn_resources::CfnResource for CfnSchema {
    fn type_string(&self) -> &'static str {
        "AWS::Personalize::Schema"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.schema;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10000 as _ {
                return Err(format!(
                    "Max validation failed on field 'schema'. {} is greater than 10000",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
