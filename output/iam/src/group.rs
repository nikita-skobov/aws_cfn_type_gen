/// Creates a new group.
///
/// For information about the number of groups you can create, see Limitations       on IAM Entities in the IAM User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGroup {
    ///
    /// The name of the group to create. Do not include the path in this value.
    ///
    /// The group name must be unique within the account. Group names are not distinguished by     case. For example, you cannot create groups named both "ADMINS" and "admins". If you don't     specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for     the group name.
    ///
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this       resource. You can perform updates that require no or some interruption. If you must       replace the resource, specify a new name.
    ///
    /// If you specify a name, you must specify the CAPABILITY_NAMED_IAM value to     acknowledge your template's capabilities. For more information, see Acknowledging IAM Resources in AWS CloudFormation     Templates.
    ///
    /// ImportantNaming an IAM resource can cause an unrecoverable error if you reuse       the same template in multiple Regions. To prevent this, we recommend using        Fn::Join and AWS::Region to create a Region-specific name,       as in the following example: {"Fn::Join": ["", [{"Ref": "AWS::Region"}, {"Ref":        "MyResourceName"}]]}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM policy you want to attach.
    ///
    /// For more information about ARNs, see Amazon Resource Names (ARNs) in the         AWS General Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedPolicyArns")]
    pub managed_policy_arns: Option<Vec<String>>,

    ///
    /// The path to the group. For more information about paths, see IAM         identifiers in the IAM User Guide.
    ///
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: (\u002F)|(\u002F[\u0021-\u007E]+\u002F)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,

    ///
    /// Adds or updates an inline policy document that is embedded in the specified IAM group. To view AWS::IAM::Group snippets, see Declaring       an IAM Group Resource.
    ///
    /// ImportantThe name of each inline policy for a role, user, or group must be unique. If you       don't choose unique names, updates to the IAM identity will fail.
    ///
    /// For information about limits on the number of inline policies that you can embed in a     group, see Limitations on IAM Entities in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Policy
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policy>>,
}

impl cfn_resources::CfnResource for CfnGroup {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::Group"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.path {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'path'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.path {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'path'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Contains information about an attached policy.
///
/// An attached policy is a managed policy that has been attached to a user, group, or     role.
///
/// For more information about managed policies, refer to Managed Policies and Inline       Policies in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Policy {
    ///
    /// The entire contents of the policy that defines permissions. For more information, see       Overview of JSON       policies.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

    ///
    /// The friendly name (not ARN) identifying the policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

impl cfn_resources::CfnResource for Policy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.policy_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'policy_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.policy_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'policy_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
