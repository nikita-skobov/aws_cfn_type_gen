/// The AWS::Batch::SchedulingPolicy resource specifies the parameters for an AWS Batch  scheduling policy. For more information, see Scheduling Policies in the AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSchedulingPolicy {
    ///
    /// The fair share policy of the scheduling policy.
    ///
    /// Required: No
    ///
    /// Type: FairsharePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "FairsharePolicy")]
    pub fairshare_policy: Option<FairsharePolicy>,

    ///
    /// The name of the scheduling policy. It can be up to 128 letters long. It can contain uppercase and lowercase  letters, numbers, hyphens (-), and underscores (_).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The tags that you apply to the scheduling policy to help you categorize and organize your resources. Each tag  consists of a key and an optional value. For more information, see Tagging AWS Resources in         AWS General   Reference.
    ///
    /// These tags can be updated or removed using the TagResource and UntagResource API operations.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnSchedulingPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::Batch::SchedulingPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.fairshare_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The fair share policy for a scheduling policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FairsharePolicy {
    ///
    /// A value used to reserve some of the available maximum vCPU for fair share identifiers that  aren't already used.
    ///
    /// The reserved ratio is   (computeReservation/100)^ActiveFairShares         where         ActiveFairShares       is the number of active fair share  identifiers.
    ///
    /// For example, a computeReservation value of 50 indicates that AWS Batchreserves  50% of the maximum available vCPU if there's only one fair share identifier. It reserves 25% if  there are two fair share identifiers. It reserves 12.5% if there are three fair share  identifiers. A computeReservation value of 25 indicates that AWS Batch should reserve  25% of the maximum available vCPU if there's only one fair share identifier, 6.25% if there are  two fair share identifiers, and 1.56% if there are three fair share identifiers.
    ///
    /// The minimum value is 0 and the maximum value is 99.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeReservation")]
    pub compute_reservation: Option<f64>,

    ///
    /// The amount of time (in seconds) to use to calculate a fair share percentage for each fair  share identifier in use. A value of zero (0) indicates that only current usage is measured. The  decay allows for more recently run jobs to have more weight than jobs that ran earlier. The  maximum supported value is 604800 (1 week).
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareDecaySeconds")]
    pub share_decay_seconds: Option<f64>,

    ///
    /// An array of SharedIdentifier objects that contain the weights for the fair  share identifiers for the fair share policy. Fair share identifiers that aren't included have a  default weight of 1.0.
    ///
    /// Required: No
    ///
    /// Type: List of ShareAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareDistribution")]
    pub share_distribution: Option<Vec<ShareAttributes>>,
}

impl cfn_resources::CfnResource for FairsharePolicy {
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

/// Specifies the weights for the fair share identifiers for the fair share policy. Fair share  identifiers that aren't included have a default weight of 1.0.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShareAttributes {
    ///
    /// A fair share identifier or fair share identifier prefix. If the string ends with an asterisk  (*), this entry specifies the weight factor to use for fair share identifiers that start with  that prefix. The list of fair share identifiers in a fair share policy can't overlap. For  example, you can't have one that specifies a shareIdentifier of UserA*  and another that specifies a shareIdentifier of UserA-1.
    ///
    /// There can be no more than 500 fair share identifiers active in a job queue.
    ///
    /// The string is limited to 255 alphanumeric characters, and can be followed by an asterisk  (*).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareIdentifier")]
    pub share_identifier: Option<String>,

    ///
    /// The weight factor for the fair share identifier. The default value is 1.0. A lower value has  a higher priority for compute resources. For example, jobs that use a share identifier with a  weight factor of 0.125 (1/8) get 8 times the compute resources of jobs that use a share  identifier with a weight factor of 1.
    ///
    /// The smallest supported value is 0.0001, and the largest supported value is 999.9999.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightFactor")]
    pub weight_factor: Option<f64>,
}

impl cfn_resources::CfnResource for ShareAttributes {
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
