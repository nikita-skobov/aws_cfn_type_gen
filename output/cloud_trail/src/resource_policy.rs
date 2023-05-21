

/// Attaches a resource-based permission policy to a CloudTrail channel that is used for an integration with an event source outside of AWS. For more information about resource-based policies, see      CloudTrail resource-based policy examples      in the CloudTrail User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnResourcePolicy {


    /// 
    /// A JSON-formatted string for an AWS resource-based policy.
    /// 
    /// The following are requirements for the resource policy:
    /// 
    /// Contains only one action: cloudtrail-data:PutAuditEvents                             Contains at least one statement. The policy can have a maximum of 20 statements.                             Each statement contains at least one principal. A statement can have a maximum of 50 principals.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcePolicy")]
    pub resource_policy: serde_json::Value,


    /// 
    /// The Amazon Resource Name (ARN) of the CloudTrail channel attached to the resource-based policy.      The following is the format of a resource ARN:      arn:aws:cloudtrail:us-east-2:123456789012:channel/MyChannel.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9._/\-:]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}
