/// Specifies a portfolio.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPortfolio {
    ///
    /// The language code.
    ///
    /// jp - Japanese                        zh - Chinese
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,

    ///
    /// The description of the portfolio.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name to use for display purposes.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: String,

    ///
    /// The name of the portfolio provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProviderName")]
    pub provider_name: String,

    ///
    /// One or more tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnPortfolio {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::Portfolio"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.accept_language {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'accept_language'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 2000 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 2000",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.display_name;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'display_name'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.display_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'display_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.provider_name;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'provider_name'. {} is greater than 50",
                the_val.len()
            ));
        }

        let the_val = &self.provider_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'provider_name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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
