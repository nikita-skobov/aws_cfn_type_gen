
pub mod cfn_member {

#[derive(serde::Serialize, Default)]
pub struct CfnMember {
    /// No documentation provided by AWS
    #[serde(rename = "NetworkId")]
    pub network_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InvitationId")]
    pub invitation_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "MemberConfiguration")]
    pub member_configuration: MemberConfiguration,

}


#[derive(serde::Serialize, Default)]
pub struct MemberConfiguration {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MemberFrameworkConfiguration")]
    pub member_framework_configuration: Option<MemberFrameworkConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct MemberFabricConfiguration {
    #[serde(rename = "AdminPassword")]
    pub admin_password: String,
    #[serde(rename = "AdminUsername")]
    pub admin_username: String,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: String,
    #[serde(rename = "VotingPolicy")]
    pub voting_policy: VotingPolicy,
    #[serde(rename = "Framework")]
    pub framework: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "NetworkFrameworkConfiguration")]
    pub network_framework_configuration: Option<NetworkFrameworkConfiguration>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ApprovalThresholdPolicy {
    #[serde(rename = "ProposalDurationInHours")]
    pub proposal_duration_in_hours: Option<usize>,
    #[serde(rename = "ThresholdComparator")]
    pub threshold_comparator: Option<String>,
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkFabricConfiguration {
    #[serde(rename = "Edition")]
    pub edition: String,

}

#[derive(serde::Serialize, Default)]
pub struct VotingPolicy {
    #[serde(rename = "ApprovalThresholdPolicy")]
    pub approval_threshold_policy: Option<ApprovalThresholdPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct MemberFrameworkConfiguration {
    #[serde(rename = "MemberFabricConfiguration")]
    pub member_fabric_configuration: Option<MemberFabricConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkFrameworkConfiguration {
    #[serde(rename = "NetworkFabricConfiguration")]
    pub network_fabric_configuration: Option<NetworkFabricConfiguration>,

}


}

pub mod cfn_node {

#[derive(serde::Serialize, Default)]
pub struct CfnNode {
    /// No documentation provided by AWS
    #[serde(rename = "NetworkId")]
    pub network_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "NodeConfiguration")]
    pub node_configuration: NodeConfiguration,

}


#[derive(serde::Serialize, Default)]
pub struct NodeConfiguration {
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,

}


}
