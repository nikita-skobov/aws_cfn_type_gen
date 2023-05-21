

/// The AWS::Route53::KeySigningKey resource creates a new key-signing key (KSK) in a hosted zone. The hosted zone ID is passed as a       parameter in the KSK properties. You can specify the properties of this KSK using the Name, Status, and         KeyManagementServiceArn properties of the resource.
#[derive(Default, serde::Serialize)]
pub struct CfnKeySigningKey {


    /// 
    /// A string used to identify a key-signing key (KSK). Name can include 			numbers, letters, and underscores (_). Name must be unique for each 			key-signing key in the same hosted zone.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The unique string (ID) that is used to identify a hosted zone. For example: Z00001111A1ABCaaABC11.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,


    /// 
    /// A string that represents the current key-signing key (KSK) status.
    /// 
    /// Status can have one of the following values:
    /// 
    /// ACTIVE                  The KSK is being used for signing.                       INACTIVE                  The KSK is not being used for signing.                       DELETING                  The KSK is in the process of being deleted.                       ACTION_NEEDED                  There is a problem with the KSK that requires you to take action to 						resolve. For example, the customer managed key might have been deleted, 						or the permissions for the customer managed key might have been 						changed.                       INTERNAL_FAILURE                  There was an error during a request. Before you can continue to work with 						DNSSEC signing, including actions that involve this KSK, you must correct 						the problem. For example, you may need to activate or deactivate the 						KSK.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 150
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,


    /// 
    /// The Amazon resource name (ARN) for a customer managed customer master key (CMK) in AWS Key Management Service (AWS KMS ). The KeyManagementServiceArn must be unique for each key-signing key (KSK) in a single hosted zone. For example: arn:aws:kms:us-east-1:111122223333:key/111a2222-a11b-1ab1-2ab2-1ab21a2b3a111.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyManagementServiceArn")]
    pub key_management_service_arn: String,

}