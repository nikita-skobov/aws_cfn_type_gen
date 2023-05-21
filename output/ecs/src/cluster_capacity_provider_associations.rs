

/// The AWS::ECS::ClusterCapacityProviderAssociations resource associates one or more capacity  providers and a default capacity provider strategy with a cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClusterCapacityProviderAssociations {


    /// 
    /// The cluster the capacity provider association is the target of.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cluster")]
    pub cluster: String,


    /// 
    /// The capacity providers to associate with the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProviders")]
    pub capacity_providers: Vec<String>,


    /// 
    /// The default capacity provider strategy to associate with the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CapacityProviderStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    pub default_capacity_provider_strategy: Vec<CapacityProviderStrategy>,

}

impl cfn_resources::CfnResource for CfnClusterCapacityProviderAssociations {
    fn type_string() -> &'static str {
        "AWS::ECS::ClusterCapacityProviderAssociations"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The CapacityProviderStrategy property specifies the details of the default capacity provider  strategy for the cluster. When services or tasks are run in the cluster with no launch type or capacity provider  strategy specified, the default capacity provider strategy is used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityProviderStrategy {


    /// 
    /// The base value designates how many tasks, at a minimum, to run on the specified capacity  provider. Only one capacity provider in a capacity provider strategy can have a base defined. If  no value is specified, the default value of 0 is used.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    pub base: Option<i64>,


    /// 
    /// The short name of the capacity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: String,


    /// 
    /// The weight value designates the relative percentage of the total number of tasks launched  that should use the specified capacity provider. The weight value is taken into consideration after the   base value, if defined, is satisfied.
    /// 
    /// If no weight value is specified, the default value of 0 is used. When multiple  capacity providers are specified within a capacity provider strategy, at least one of the capacity providers must  have a weight value greater than zero and any capacity providers with a weight of 0 will not be used to  place tasks. If you specify multiple capacity providers in a strategy that all have a weight of 0, any   RunTask or CreateService actions using the capacity provider strategy will fail.
    /// 
    /// An example scenario for using weights is defining a strategy that contains two capacity providers and both have  a weight of 1, then when the base is satisfied, the tasks will be split evenly across the  two capacity providers. Using that same logic, if you specify a weight of 1 for   capacityProviderA and a weight of 4 for capacityProviderB,  then for every one task that is run using capacityProviderA, four tasks would use   capacityProviderB.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<i64>,

}
