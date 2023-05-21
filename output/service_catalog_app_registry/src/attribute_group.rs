/// Creates a new attribute group as a container for user-defined attributes. This feature    enables users to have full control over their cloud application's metadata in a rich    machine-readable format to facilitate integration with automated workflows and third-party    tools.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAttributeGroup {
    ///
    /// A nested object      in a JSON or YAML template      that supports arbitrary definitions.      Represents the attributes      in an attribute group      that describes an application and its components.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: serde_json::Value,

    ///
    /// The description of the attribute group that the user provides.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name of the attribute group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [-.\w]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Key-value pairs you can use to associate with the attribute group.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnAttributeGroup {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalogAppRegistry::AttributeGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
