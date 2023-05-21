

/// Creates a portal, which can contain projects and dashboards. Before you can create a       portal, you must enable IAM Identity Center. AWS IoT SiteWise Monitor uses IAM Identity Center to manage user permissions. For more    information, see Enabling      IAM Identity Center in the AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPortal {


    /// 
    /// Contains the configuration information of an alarm created in an AWS IoT SiteWise Monitor portal.  You can use the alarm to monitor an asset property and get notified when the asset property value is outside a specified range.  For more information, see Monitoring with alarms in the         AWS IoT SiteWise Application Guide.
    /// 
    /// Required: No
    ///
    /// Type: Alarms
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alarms")]
    pub alarms: Option<Alarms>,


    /// 
    /// The email address that sends alarm notifications.
    /// 
    /// ImportantIf you use the AWS IoT Events managed Lambda      function to manage your emails, you must verify the sender email      address in Amazon SES.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationSenderEmail")]
    pub notification_sender_email: Option<String>,


    /// 
    /// The service to use to authenticate users to the portal. Choose from the following    options:
    /// 
    /// SSO – The portal uses AWS IAM Identity Center (successor to AWS Single Sign-On) to authenticate users and manage          user permissions. Before you can create a portal that uses IAM Identity Center, you must enable IAM Identity Center.          For more information, see Enabling IAM Identity Center in the          AWS IoT SiteWise User Guide. This option is only available in AWS Regions other than      the China Regions.                         IAM – The portal uses AWS Identity and Access Management (IAM) to authenticate users and manage          user permissions.
    /// 
    /// You can't change this value after you create a portal.
    /// 
    /// Default: SSO
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PortalAuthMode")]
    pub portal_auth_mode: Option<String>,


    /// 
    /// The AWS administrator's contact email address.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortalContactEmail")]
    pub portal_contact_email: String,


    /// 
    /// A description for the portal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortalDescription")]
    pub portal_description: Option<String>,


    /// 
    /// A friendly name for the portal.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortalName")]
    pub portal_name: String,


    /// 
    /// The ARN of a service role that allows the portal's users to access your AWS IoT SiteWise       resources on your behalf. For more information, see Using service roles for AWS IoT SiteWise Monitor in the       AWS IoT SiteWise User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// A list of key-value pairs that contain metadata for the portal. For more information, see       Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnPortal {
    fn type_string() -> &'static str {
        "AWS::IoTSiteWise::Portal"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains the configuration information of an alarm created in an AWS IoT SiteWise Monitor portal.  You can use the alarm to monitor an asset property and get notified when the asset property value is outside a specified range.  For more information, see Monitoring with alarms in the         AWS IoT SiteWise Application Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Alarms {


    /// 
    /// The ARN of the IAM role that allows the alarm to perform actions and access AWS    resources and services, such as AWS IoT Events.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmRoleArn")]
    pub alarm_role_arn: Option<String>,


    /// 
    /// The ARN of the Lambda function that manages alarm notifications. For more    information, see Managing alarm     notifications in the         AWS IoT Events Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationLambdaArn")]
    pub notification_lambda_arn: Option<String>,

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


