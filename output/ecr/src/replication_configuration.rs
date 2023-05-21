

/// The AWS::ECR::ReplicationConfiguration resource creates or updates the       replication configuration for a private registry. The first time a replication       configuration is applied to a private registry, a service-linked IAM role       is created in your account for the replication process. For more information, see Using         Service-Linked Roles for Amazon ECR in the Amazon Elastic         Container Registry User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnReplicationConfiguration {


    /// 
    /// The replication configuration for a registry.
    /// 
    /// Required: Yes
    ///
    /// Type: ReplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: Box<ReplicationConfiguration>,

}



impl cfn_resources::CfnResource for CfnReplicationConfiguration {
    fn type_string() -> &'static str {
        "AWS::ECR::ReplicationConfiguration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// An array of objects representing the destination for a replication rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationDestination {


    /// 
    /// The AWS account ID of the Amazon ECR private registry to replicate to. When configuring       cross-Region replication within your own registry, specify your own account ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [0-9]{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryId")]
    pub registry_id: String,


    /// 
    /// The Region to replicate to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 25
    ///
    /// Pattern: [0-9a-z-]{2,25}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,

}




/// The filter settings used with image replication. Specifying a repository filter to a       replication rule provides a method for controlling which repositories in a private       registry are replicated. If no filters are added, the contents of all repositories are       replicated.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RepositoryFilter {


    /// 
    /// The repository filter type. The only supported value is PREFIX_MATCH,       which is a repository name prefix specified with the filter       parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PREFIX_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterType")]
    pub filter_type: RepositoryFilterFilterTypeEnum,


    /// 
    /// The repository filter details. When the PREFIX_MATCH filter type is       specified, this value is required and should be the repository name prefix to configure       replication for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(?:[a-z0-9]+(?:[._-][a-z0-9]*)*/)*[a-z0-9]*(?:[._-][a-z0-9]*)*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    pub filter: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RepositoryFilterFilterTypeEnum {

    /// PREFIX_MATCH
    #[serde(rename = "PREFIX_MATCH")]
    Prefixmatch,

}

impl Default for RepositoryFilterFilterTypeEnum {
    fn default() -> Self {
        RepositoryFilterFilterTypeEnum::Prefixmatch
    }
}



/// An array of objects representing the replication destinations and repository filters       for a replication configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationRule {


    /// 
    /// An array of objects representing the filters for a replication rule. Specifying a       repository filter for a replication rule provides a method for controlling which       repositories in a private registry are replicated.
    /// 
    /// Required: No
    ///
    /// Type: List of RepositoryFilter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryFilters")]
    pub repository_filters: Option<Vec<RepositoryFilter>>,


    /// 
    /// An array of objects representing the destination for a replication rule.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ReplicationDestination
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Vec<ReplicationDestination>,

}




/// The replication configuration for a registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationConfiguration {


    /// 
    /// An array of objects representing the replication destinations and repository filters       for a replication configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ReplicationRule
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<ReplicationRule>,

}


