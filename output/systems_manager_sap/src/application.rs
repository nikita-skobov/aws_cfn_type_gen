/// An SAP application registered with AWS Systems Manager for 			SAP.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApplication {
    ///
    /// The ID of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [\w\d]{1,50}
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationId")]
    pub application_id: cfn_resources::StrVal,

    ///
    /// The type of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: HANA
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationType")]
    pub application_type: ApplicationApplicationTypeEnum,

    ///
    /// The credentials of the SAP application.
    ///
    /// Required: No
    ///
    /// Type: List of Credential
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<Credential>>,

    ///
    /// The Amazon EC2 instances on which your SAP application is running.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,

    ///
    /// The SAP instance number of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [0-9]{2}
    ///
    /// Update requires: Replacement
    #[serde(rename = "SapInstanceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sap_instance_number: Option<cfn_resources::StrVal>,

    ///
    /// The System ID of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [A-Z][A-Z0-9]{2}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<cfn_resources::StrVal>,

    ///
    /// The tags on the application.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnApplicationarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ApplicationApplicationTypeEnum {
    /// HANA
    #[serde(rename = "HANA")]
    Hana,
}

impl Default for ApplicationApplicationTypeEnum {
    fn default() -> Self {
        ApplicationApplicationTypeEnum::Hana
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApplicationarn;
impl CfnApplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::SystemsManagerSAP::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.credentials {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'credentials'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.instances {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'instances'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The credentials of your SAP application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Credential {
    ///
    /// The type of the application credentials.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CredentialType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<cfn_resources::StrVal>,

    ///
    /// The name of the SAP HANA database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// The secret ID created in AWS Secrets Manager to store the credentials 			of the SAP application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Credential {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
