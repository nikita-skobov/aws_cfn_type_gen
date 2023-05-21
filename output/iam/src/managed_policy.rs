

/// Creates a new managed policy for your AWS account.
///
/// This operation creates a policy version with a version identifier of v1       and sets v1 as the policy's default version. For more information about policy versions,       see Versioning for managed policies in the         IAM User Guide.
///
/// As a best practice, you can validate your IAM policies.    To learn more, see Validating IAM policies       in the IAM User Guide.
///
/// For more information about managed policies in general, see Managed         policies and inline policies in the       IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnManagedPolicy {


    /// 
    /// A friendly description of the policy.
    /// 
    /// Typically used to store information about the permissions defined in the policy. For       example, "Grants access to production DynamoDB tables."
    /// 
    /// The policy description is immutable. After a value is assigned, it cannot be       changed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name (friendly name, not ARN) of the group to attach the policy to.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,


    /// 
    /// The friendly name of the policy.
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
    #[serde(rename = "ManagedPolicyName")]
    pub managed_policy_name: Option<String>,


    /// 
    /// The path for the policy.
    /// 
    /// For more information about paths, see IAM identifiers in the         IAM User Guide.
    /// 
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    /// 
    /// NoteYou cannot use an asterisk (*) in the path name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ((/[A-Za-z0-9\.,\+@=_-]+)*)/
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    pub path: Option<String>,


    /// 
    /// The JSON policy document that you want to use as the content for the new       policy.
    /// 
    /// You must provide policies in JSON format in IAM. However, for AWS CloudFormation       templates formatted in YAML, you can provide the policy in JSON or YAML format. AWS CloudFormation always converts a YAML policy to JSON format before submitting it to       IAM.
    /// 
    /// The maximum length of the policy document that you can pass in this operation,       including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see IAM and AWS STS character quotas.
    /// 
    /// To learn more about JSON policy grammar, see Grammar of the IAM JSON         policy language in the IAM User Guide.
    /// 
    /// The regex pattern   used to validate this parameter is a string of characters consisting of the following:
    /// 
    /// Any printable ASCII   character ranging from the space character (\u0020) through the end of the ASCII character range               The printable characters in the Basic Latin and Latin-1 Supplement character set   (through \u00FF)               The special characters tab (\u0009), line feed (\u000A), and   carriage return (\u000D)
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 131072
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u00FF]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,


    /// 
    /// The name (friendly name, not ARN) of the role to attach the policy to.
    /// 
    /// This parameter allows (per its regex       pattern) a string of characters consisting of upper and lowercase alphanumeric     characters with no spaces. You can also include any of the following characters:     _+=,.@-
    /// 
    /// NoteIf an external policy (such as AWS::IAM::Policy or        AWS::IAM::ManagedPolicy) has a Ref to a role and if a       resource (such as AWS::ECS::Service) also has a Ref to the       same role, add a DependsOn attribute to the resource to make the resource       depend on the external policy. This dependency ensures that the role's policy is       available throughout the resource's lifecycle. For example, when you delete a stack with       an AWS::ECS::Service resource, the DependsOn attribute ensures       that AWS CloudFormation deletes the AWS::ECS::Service resource before       deleting its role's policy.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<String>>,


    /// 
    /// The name (friendly name, not ARN) of the IAM user to attach the policy to.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for CfnManagedPolicy {
    fn type_string() -> &'static str {
        "AWS::IAM::ManagedPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
