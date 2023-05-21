/// Creates a new rule set for FlexMatch matchmaking. A rule set describes the type of match       to create, such as the number and size of teams. It also sets the parameters for       acceptable player matches, such as minimum skill level or character type.
///
/// To create a matchmaking rule set, provide unique rule set name and the rule set body       in JSON format. Rule sets must be defined in the same Region as the matchmaking       configuration they are used with.
///
/// Since matchmaking rule sets cannot be edited, it is a good idea to check the rule       set syntax.
///
/// Learn more
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMatchmakingRuleSet {
    ///
    /// A unique identifier for the matchmaking rule set. A matchmaking configuration identifies the rule set it uses by this name       value. Note that the rule set name is different from the optional name       field in the rule set body.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9-\.]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// A collection of matchmaking rules, formatted as a JSON string. Comments are not       allowed in JSON, but most elements support a description field.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleSetBody")]
    pub rule_set_body: String,

    ///
    /// A list of labels to assign to the new matchmaking rule set resource. Tags are developer-defined    key-value pairs. Tagging    AWS resources are useful for resource management, access management and cost allocation.    For more information, see Tagging AWS Resources in the        AWS General Reference. Once the resource is created, you can    use TagResource, UntagResource, and    ListTagsForResource to add, remove, and view tags. The    maximum tag limit may be lower than stated. See the AWS General Reference for actual    tagging limits.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnMatchmakingRuleSet {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::MatchmakingRuleSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.rule_set_body;

        if the_val.len() > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'rule_set_body'. {} is greater than 65535",
                the_val.len()
            ));
        }

        let the_val = &self.rule_set_body;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'rule_set_body'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
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
