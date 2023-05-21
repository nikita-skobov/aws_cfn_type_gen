

/// Creates a member within a Managed Blockchain network.
///
/// Applies only to Hyperledger Fabric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMember {


    /// 
    /// The unique identifier of the invitation to join the network sent to the account that creates the member.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvitationId")]
    pub invitation_id: Option<String>,


    /// 
    /// Configuration properties of the member.
    /// 
    /// Required: Yes
    ///
    /// Type: MemberConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberConfiguration")]
    pub member_configuration: MemberConfiguration,


    /// 
    /// Configuration properties of the network to which the member belongs.
    /// 
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,


    /// 
    /// The unique identifier of the network to which the member belongs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkId")]
    pub network_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnMember {
    fn type_string() -> &'static str {
        "AWS::ManagedBlockchain::Member"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.invitation_id {

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'invitation_id'. {} is greater than 32", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.invitation_id {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'invitation_id'. {} is less than 1", the_val.len()));
        }

        }
        
        self.member_configuration.validate()?;

        self.network_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A policy type that defines the voting rules for the network. The rules decide if a proposal is approved. Approval may be based on criteria such as the percentage of YES votes and the duration of the proposal. The policy applies to all proposals and is specified when the network is created.
///
/// Applies only to Hyperledger Fabric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApprovalThresholdPolicy {


    /// 
    /// The duration from the time that a proposal is created until it expires. If members cast neither the required number of YES votes to approve the proposal nor the number of NO votes required to reject it before the duration expires, the proposal is EXPIRED and ProposalActions aren't carried out.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 168
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProposalDurationInHours")]
    pub proposal_duration_in_hours: Option<i64>,


    /// 
    /// Determines whether the vote percentage must be greater than the ThresholdPercentage or must be greater than or equal to the ThreholdPercentage to be approved.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GREATER_THAN | GREATER_THAN_OR_EQUAL_TO
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdComparator")]
    pub threshold_comparator: Option<ApprovalThresholdPolicyThresholdComparatorEnum>,


    /// 
    /// The percentage of votes among all members that must be YES for a proposal to be approved. For example, a ThresholdPercentage value of 50 indicates 50%. The ThresholdComparator determines the precise comparison. If a ThresholdPercentage value of 50 is specified on a network with 10 members, along with a ThresholdComparator value of GREATER_THAN, this indicates that 6 YES votes are required for the proposal to be approved.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApprovalThresholdPolicyThresholdComparatorEnum {

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

}

impl Default for ApprovalThresholdPolicyThresholdComparatorEnum {
    fn default() -> Self {
        ApprovalThresholdPolicyThresholdComparatorEnum::Greaterthan
    }
}


impl cfn_resources::CfnResource for ApprovalThresholdPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.proposal_duration_in_hours {

        if *the_val > 168 as _ {
            return Err(format!("Max validation failed on field 'proposal_duration_in_hours'. {} is greater than 168", the_val));
        }

        }
        
        if let Some(the_val) = &self.proposal_duration_in_hours {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'proposal_duration_in_hours'. {} is less than 1", the_val));
        }

        }
        
        if let Some(the_val) = &self.threshold_percentage {

        if *the_val > 100 as _ {
            return Err(format!("Max validation failed on field 'threshold_percentage'. {} is greater than 100", the_val));
        }

        }
        
        if let Some(the_val) = &self.threshold_percentage {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'threshold_percentage'. {} is less than 0", the_val));
        }

        }
        
        Ok(())
    }
}

/// Configuration properties of the member.
///
/// Applies only to Hyperledger Fabric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemberConfiguration {


    /// 
    /// An optional description of the member.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Configuration properties of the blockchain framework relevant to the member.
    /// 
    /// Required: No
    ///
    /// Type: MemberFrameworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberFrameworkConfiguration")]
    pub member_framework_configuration: Option<MemberFrameworkConfiguration>,


    /// 
    /// The name of the member.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^(?!-|[0-9])(?!.*-$)(?!.*?--)[a-zA-Z0-9-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for MemberConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 128", the_val.len()));
        }

        }
        
        self.member_framework_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Configuration properties for Hyperledger Fabric for a member in a Managed Blockchain network that is using the Hyperledger Fabric framework.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemberFabricConfiguration {


    /// 
    /// The password for the member's initial administrative user. The AdminPassword must be at least 8 characters long and no more than 32 characters. It must contain at least one uppercase letter, one lowercase letter, and one digit. It cannot have a single quotation mark (‘), a double quotation marks (“), a forward slash(/), a backward slash(\), @, or a space.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 32
    ///
    /// Pattern: ^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?!.*[@'\\"/])[a-zA-Z0-9\S]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminPassword")]
    pub admin_password: String,


    /// 
    /// The user name for the member's initial administrative user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUsername")]
    pub admin_username: String,

}



impl cfn_resources::CfnResource for MemberFabricConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.admin_password;

        if the_val.len() > 32 as _ {
            return Err(format!("Max validation failed on field 'admin_password'. {} is greater than 32", the_val.len()));
        }

        
        let the_val = &self.admin_password;

        if the_val.len() < 8 as _ {
            return Err(format!("Min validation failed on field 'admin_password'. {} is less than 8", the_val.len()));
        }

        
        let the_val = &self.admin_username;

        if the_val.len() > 16 as _ {
            return Err(format!("Max validation failed on field 'admin_username'. {} is greater than 16", the_val.len()));
        }

        
        let the_val = &self.admin_username;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'admin_username'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Configuration properties relevant to a member for the blockchain framework that the Managed Blockchain network uses.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MemberFrameworkConfiguration {


    /// Configuration properties for Hyperledger Fabric.
    ///
    /// Required: No
    ///
    /// Type: MemberFabricConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberFabricConfiguration")]
    pub member_fabric_configuration: Option<MemberFabricConfiguration>,

}



impl cfn_resources::CfnResource for MemberFrameworkConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.member_fabric_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration properties of the network to which the member belongs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {


    /// 
    /// Attributes of the blockchain framework for the network.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The blockchain framework that the network uses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ETHEREUM | HYPERLEDGER_FABRIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Framework")]
    pub framework: NetworkConfigurationFrameworkEnum,


    /// 
    /// The version of the blockchain framework that the network uses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 8
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: String,


    /// 
    /// The name of the network.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Configuration properties relevant to the network for the blockchain framework that the network uses.
    ///
    /// Required: No
    ///
    /// Type: NetworkFrameworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkFrameworkConfiguration")]
    pub network_framework_configuration: Option<NetworkFrameworkConfiguration>,


    /// 
    /// The voting rules that the network uses to decide if a proposal is accepted.
    /// 
    /// Required: Yes
    ///
    /// Type: VotingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "VotingPolicy")]
    pub voting_policy: VotingPolicy,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkConfigurationFrameworkEnum {

    /// ETHEREUM
    #[serde(rename = "ETHEREUM")]
    Ethereum,

    /// HYPERLEDGER_FABRIC
    #[serde(rename = "HYPERLEDGER_FABRIC")]
    Hyperledgerfabric,

}

impl Default for NetworkConfigurationFrameworkEnum {
    fn default() -> Self {
        NetworkConfigurationFrameworkEnum::Ethereum
    }
}


impl cfn_resources::CfnResource for NetworkConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 128", the_val.len()));
        }

        }
        
        let the_val = &self.framework_version;

        if the_val.len() > 8 as _ {
            return Err(format!("Max validation failed on field 'framework_version'. {} is greater than 8", the_val.len()));
        }

        
        let the_val = &self.framework_version;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'framework_version'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        self.network_framework_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.voting_policy.validate()?;

        Ok(())
    }
}

/// Hyperledger Fabric configuration properties for the network.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkFabricConfiguration {


    /// 
    /// The edition of Amazon Managed Blockchain that the network uses. Valid values are       standard and starter. For more information, see Amazon Managed Blockchain Pricing
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: STANDARD | STARTER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Edition")]
    pub edition: NetworkFabricConfigurationEditionEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkFabricConfigurationEditionEnum {

    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

    /// STARTER
    #[serde(rename = "STARTER")]
    Starter,

}

impl Default for NetworkFabricConfigurationEditionEnum {
    fn default() -> Self {
        NetworkFabricConfigurationEditionEnum::Standard
    }
}


impl cfn_resources::CfnResource for NetworkFabricConfiguration {
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

/// Configuration properties relevant to the network for the blockchain framework that the network uses.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkFrameworkConfiguration {


    /// 
    /// Configuration properties for Hyperledger Fabric for a member in a Managed Blockchain network that is using the Hyperledger Fabric framework.
    /// 
    /// Required: No
    ///
    /// Type: NetworkFabricConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkFabricConfiguration")]
    pub network_fabric_configuration: Option<NetworkFabricConfiguration>,

}



impl cfn_resources::CfnResource for NetworkFrameworkConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.network_fabric_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The voting rules for the network to decide if a proposal is accepted
///
/// Applies only to Hyperledger Fabric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VotingPolicy {


    /// 
    /// Defines the rules for the network for voting on proposals, such as the percentage of YES votes required for the proposal to be approved and the duration of the proposal. The policy applies to all proposals and is specified when the network is created.
    /// 
    /// Required: No
    ///
    /// Type: ApprovalThresholdPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalThresholdPolicy")]
    pub approval_threshold_policy: Option<ApprovalThresholdPolicy>,

}



impl cfn_resources::CfnResource for VotingPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.approval_threshold_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}