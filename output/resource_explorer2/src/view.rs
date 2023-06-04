/// Creates a view that users can query by using the Search       operation. Results from queries that you make using this view include only resources       that match the view's Filters.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnView {
    ///
    /// An array of strings that include search keywords, prefixes, and operators that filter       the results that are returned for queries made using this view. When you use this view       in a Search       operation, the filter string is combined with the search's QueryString       parameter using a logical AND operator.
    ///
    /// For information about the supported syntax, see Search query         reference for Resource Explorer in the AWS         Resource Explorer User Guide.
    ///
    /// ImportantThis query string in the context of this operation supports only filter prefixes with optional operators. It doesn't support free-form text. For example, the string           region:us* service:ec2 -tag:stage=prod includes all Amazon EC2 resources in any AWS Region that begin with the         letters us and are not tagged with a key           Stage that has the value prod.
    ///
    /// Required: No
    ///
    /// Type: Filters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,

    ///
    /// A list of fields that provide additional information about the view.
    ///
    /// Required: No
    ///
    /// Type: List of IncludedProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_properties: Option<Vec<IncludedProperty>>,

    ///
    /// Tag key and value pairs that are attached to the view.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The name of the new view.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9\-]{1,64}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ViewName")]
    pub view_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_view_arn: CfnViewviewarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnViewviewarn;
impl CfnViewviewarn {
    pub fn att_name(&self) -> &'static str {
        r#"ViewArn"#
    }
}

impl cfn_resources::CfnResource for CfnView {
    fn type_string(&self) -> &'static str {
        "AWS::ResourceExplorer2::View"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filters.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object with a FilterString that specifies which resources to include       in the results of queries made using this view.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Filters {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterString")]
    pub filter_string: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Filters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information about an additional property that describes a resource, that you can       optionally include in a view.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IncludedProperty {
    ///
    /// The name of the property that is included in this view.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IncludedProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1011 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 1011",
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

        Ok(())
    }
}
