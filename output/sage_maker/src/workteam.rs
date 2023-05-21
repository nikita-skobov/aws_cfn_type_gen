

/// Creates a new work team for labeling your data. A work team is defined by one or more       Amazon Cognito user pools. You must first create the user pools before you can create a work       team.
///
/// You cannot create more than 25 work teams in an account and region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWorkteam {


    /// 
    /// A list of MemberDefinition objects that contains objects that identify       the workers that make up the work team.
    /// 
    /// Workforces can be created using Amazon Cognito or your own OIDC Identity Provider (IdP).       For private workforces created using Amazon Cognito use       CognitoMemberDefinition. For workforces created using your own OIDC identity       provider (IdP) use OidcMemberDefinition.
    /// 
    /// Required: No
    ///
    /// Type: List of MemberDefinition
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberDefinitions")]
    pub member_definitions: Option<Vec<MemberDefinition>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkforceName")]
    pub workforce_name: Option<String>,


    /// 
    /// Configures SNS notifications of available or expiring work items for work       teams.
    /// 
    /// Required: No
    ///
    /// Type: NotificationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: Option<NotificationConfiguration>,


    /// 
    /// A description of the work team.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: .+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the work team.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkteamName")]
    pub workteam_name: Option<String>,


    /// 
    /// An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

impl cfn_resources::CfnResource for CfnWorkteam {
    fn type_string() -> &'static str {
        "AWS::SageMaker::Workteam"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A list of user groups that exist in your OIDC Identity Provider (IdP).       One to ten groups can be used to create a single private work team.       When you add a user group to the list of Groups, you can add that user group to one or more       private work teams. If you add a user group to a private work team, all workers in that user group       are added to the work team.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OidcMemberDefinition {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OidcGroups")]
    pub oidc_groups: Vec<String>,

}


/// Identifies a Amazon Cognito user group. A user group can be used in on or more work       teams.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CognitoMemberDefinition {


    /// 
    /// An identifier for an application client. You must create the app client ID using       Amazon Cognito.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoClientId")]
    pub cognito_client_id: String,


    /// 
    /// An identifier for a user group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoUserGroup")]
    pub cognito_user_group: String,


    /// 
    /// An identifier for a user pool. The user pool must be in the same region as the service       that you are calling.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CognitoUserPool")]
    pub cognito_user_pool: String,

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


/// Defines an Amazon Cognito or your own OIDC IdP user group that is part of a work team.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemberDefinition {


    /// 
    /// The Amazon Cognito user group that is part of the work team.
    /// 
    /// Required: No
    ///
    /// Type: CognitoMemberDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoMemberDefinition")]
    pub cognito_member_definition: Option<CognitoMemberDefinition>,


    /// 
    /// A list user groups that exist in your OIDC Identity Provider (IdP).       One to ten groups can be used to create a single private work team.       When you add a user group to the list of Groups, you can add that user group to one or more       private work teams. If you add a user group to a private work team, all workers in that user group       are added to the work team.
    /// 
    /// Required: No
    ///
    /// Type: OidcMemberDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "OidcMemberDefinition")]
    pub oidc_member_definition: Option<OidcMemberDefinition>,

}


/// Configures Amazon SNS notifications of available or expiring work items for work       teams.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationConfiguration {


    /// 
    /// The ARN for the Amazon SNS topic to which notifications should be published.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: arn:aws[a-z\-]*:sns:[a-z0-9\-]*:[0-9]{12}:[a-zA-Z0-9_.-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTopicArn")]
    pub notification_topic_arn: String,

}
