

/// Running PutPermission permits the specified AWS account or AWS organization    to put events to the specified event bus. Amazon EventBridge (CloudWatch    Events) rules in your account are triggered by these events arriving to an event bus in your    account.
///
/// For another account to send events to your account, that external account must have an    EventBridge rule with your account's event bus as a target.
///
/// To enable multiple AWS accounts to put events to your event bus, run     PutPermission once for each of these accounts. Or, if all the accounts are    members of the same AWS organization, you can run PutPermission once specifying     Principal as "*" and specifying the AWS organization ID in     Condition, to grant permissions to all accounts in that organization.
///
/// If you grant permissions using an organization, then accounts in that organization must    specify a RoleArn with proper permissions when they use PutTarget to    add your account's event bus as a target. For more information, see Sending and     Receiving Events Between AWS Accounts in the Amazon EventBridge User     Guide.
///
/// The permission policy on the event bus cannot exceed 10 KB in size.
#[derive(Default, serde::Serialize)]
pub struct CfnEventBusPolicy {


    /// 
    /// The 12-digit AWS account ID that you are permitting to put events to your default event    bus. Specify "*" to permit any account to put events to your default event bus.
    /// 
    /// If you specify "*" without specifying Condition, avoid creating rules that    may match undesirable events. To create more secure rules, make sure that the event pattern    for each rule contains an account field with a specific account ID from which to    receive events. Rules with an account field do not match any events sent from other    accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 12
    ///
    /// Pattern: (\d{12}|\*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principal")]
    pub principal: Option<String>,


    /// 
    /// A JSON string that describes the permission policy statement. You can include a     Policy parameter in the request instead of using the StatementId,     Action, Principal, or Condition parameters.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statement")]
    pub statement: Option<serde_json::Value>,


    /// 
    /// This parameter enables you to limit the permission to accounts that fulfill a certain    condition, such as being a member of a certain AWS organization. For more information about    AWS Organizations, see What Is AWS     Organizations in the         AWS Organizations User Guide.
    /// 
    /// If you specify Condition with an AWS organization ID, and specify "*" as the    value for Principal, you grant permission to all the accounts in the named    organization.
    /// 
    /// The Condition is a JSON string which must contain Type,     Key, and Value fields.
    /// 
    /// Required: No
    ///
    /// Type: Condition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: Option<Condition>,


    /// 
    /// The name of the event bus associated with the rule. If you omit this, the default event    bus is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,


    /// 
    /// An identifier string for the external account that you are granting permissions to. If you    later want to revoke the permission for this external account, specify this    StatementId when you run RemovePermission.
    /// 
    /// NoteEach StatementId must be unique.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [a-zA-Z0-9-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatementId")]
    pub statement_id: String,


    /// 
    /// The action that you are enabling the other account to perform.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: events:[a-zA-Z]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<String>,

}


/// A JSON string which you can use to limit the event bus permissions you are granting to    only accounts that fulfill the condition. Currently, the only supported condition is    membership in a certain AWS organization. The string must contain Type,     Key, and Value fields. The Value field specifies the    ID of the AWS organization. Following is an example value for Condition:
///
/// '{"Type" : "StringEquals", "Key": "aws:PrincipalOrgID", "Value":     "o-1234567890"}'
#[derive(Default, serde::Serialize)]
pub struct Condition {


    /// 
    /// Specifies the key for the condition. Currently the only supported key is     aws:PrincipalOrgID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// Specifies the type of condition. Currently the only supported value is     StringEquals.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// Specifies the value for the key. Currently, this must be the ID of the    organization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}