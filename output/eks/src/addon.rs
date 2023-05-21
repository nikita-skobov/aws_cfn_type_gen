

/// Creates an Amazon EKS add-on.
///
/// Amazon EKS add-ons help to automate the provisioning and lifecycle management       of common operational software for Amazon EKS clusters. For more information,       see Amazon EKS add-ons in the Amazon EKS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAddon {


    /// 
    /// The name of the add-on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AddonName")]
    pub addon_name: String,


    /// 
    /// The version of the add-on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddonVersion")]
    pub addon_version: Option<String>,


    /// 
    /// The name of the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z][A-Za-z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,


    /// 
    /// The configuration values that you provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationValues")]
    pub configuration_values: Option<String>,


    /// 
    /// Specifying this option preserves the add-on software on your cluster but Amazon EKS stops managing any settings for the add-on. If an IAM       account is associated with the add-on, it isn't removed.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveOnDelete")]
    pub preserve_on_delete: Option<bool>,


    /// 
    /// How to resolve field value conflicts for an Amazon EKS add-on. Conflicts are       handled based on the value you choose:
    /// 
    /// None – If the self-managed version of           the add-on is installed on your cluster, Amazon EKS doesn't change the           value. Creation of the add-on might fail.                        Overwrite – If the self-managed           version of the add-on is installed on your cluster and the Amazon EKS           default value is different than the existing value, Amazon EKS changes           the value to the Amazon EKS default value.                        Preserve – Not supported. You can set           this value when updating an add-on though. For more information, see UpdateAddon.
    /// 
    /// If you don't currently have the self-managed version of the add-on installed on your       cluster, the Amazon EKS add-on is installed. Amazon EKS sets all values       to default values, regardless of the option that you specify.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | OVERWRITE | PRESERVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResolveConflicts")]
    pub resolve_conflicts: Option<AddonResolveConflictsEnum>,


    /// 
    /// The Amazon Resource Name (ARN) of an existing IAM role to bind to the add-on's service account. The role must be assigned the IAM permissions required by the add-on. If you don't specify an existing IAM role, then the add-on uses the   permissions assigned to the node IAM role. For more information, see Amazon EKS node IAM role in the Amazon EKS User Guide.
    /// 
    /// NoteTo specify an existing IAM role, you must have an IAM OpenID Connect (OIDC) provider created for         your cluster. For more information, see Enabling           IAM roles for service accounts on your cluster in the         Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccountRoleArn")]
    pub service_account_role_arn: Option<String>,


    /// 
    /// The metadata that you apply to the add-on to assist with categorization and       organization. Each tag consists of a key and an optional value, both of which you       define. Add-on tags do not propagate to any other resources associated with the       cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AddonResolveConflictsEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// OVERWRITE
    #[serde(rename = "OVERWRITE")]
    Overwrite,

    /// PRESERVE
    #[serde(rename = "PRESERVE")]
    Preserve,

}

impl Default for AddonResolveConflictsEnum {
    fn default() -> Self {
        AddonResolveConflictsEnum::None
    }
}


impl cfn_resources::CfnResource for CfnAddon {
    fn type_string() -> &'static str {
        "AWS::EKS::Addon"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cluster_name;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'cluster_name'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.cluster_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'cluster_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.service_account_role_arn {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'service_account_role_arn'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.service_account_role_arn {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'service_account_role_arn'. {} is less than 1", the_val.len()));
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}