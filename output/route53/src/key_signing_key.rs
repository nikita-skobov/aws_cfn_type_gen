/// The AWS::Route53::KeySigningKey resource creates a new key-signing key (KSK) in a hosted zone. The hosted zone ID is passed as a       parameter in the KSK properties. You can specify the properties of this KSK using the Name, Status, and         KeyManagementServiceArn properties of the resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnKeySigningKey {
    ///
    /// The unique string (ID) that is used to identify a hosted zone. For example: Z00001111A1ABCaaABC11.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: cfn_resources::StrVal,

    ///
    /// The Amazon resource name (ARN) for a customer managed customer master key (CMK) in AWS Key Management Service (AWS KMS ). The KeyManagementServiceArn must be unique for each key-signing key (KSK) in a single hosted zone. For example: arn:aws:kms:us-east-1:111122223333:key/111a2222-a11b-1ab1-2ab2-1ab21a2b3a111.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyManagementServiceArn")]
    pub key_management_service_arn: cfn_resources::StrVal,

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
    pub name: cfn_resources::StrVal,

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
    pub status: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnKeySigningKey {
    fn type_string(&self) -> &'static str {
        "AWS::Route53::KeySigningKey"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 3",
                    s.len()
                ));
            }
        }

        let the_val = &self.status;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 150 as _ {
                return Err(format!(
                    "Max validation failed on field 'status'. {} is greater than 150",
                    s.len()
                ));
            }
        }

        let the_val = &self.status;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 5 as _ {
                return Err(format!(
                    "Min validation failed on field 'status'. {} is less than 5",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
