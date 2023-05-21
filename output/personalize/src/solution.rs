

/// An object that provides information about a solution. A solution is a trained model    that can be deployed as a campaign.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSolution {


    /// 
    /// The Amazon Resource Name (ARN) of the dataset group that provides the training data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,


    /// 
    /// The event type (for example, 'click' or 'like') that is used for training the model.    If no eventType is provided, Amazon Personalize uses all interactions for training with    equal weight regardless of type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventType")]
    pub event_type: Option<String>,


    /// 
    /// The name of the solution.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// ImportantWe don't recommend enabling automated machine learning. Instead, match your use case to the available Amazon Personalize     recipes. For more information, see Determining your use case.
    /// 
    /// When true, Amazon Personalize performs a search for the best USER_PERSONALIZATION recipe from    the list specified in the solution configuration (recipeArn must not be specified).    When false (the default), Amazon Personalize uses recipeArn for training.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PerformAutoML")]
    pub perform_auto_ml: Option<bool>,


    /// 
    /// Whether to perform hyperparameter optimization (HPO) on the chosen recipe. The    default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PerformHPO")]
    pub perform_hpo: Option<bool>,


    /// 
    /// The ARN of the recipe used to create the solution.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "RecipeArn")]
    pub recipe_arn: Option<String>,


    /// 
    /// Describes the configuration properties for the solution.
    /// 
    /// Required: No
    ///
    /// Type: SolutionConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "SolutionConfig")]
    pub solution_config: Option<SolutionConfig>,

}



impl cfn_resources::CfnResource for CfnSolution {
    fn type_string() -> &'static str {
        "AWS::Personalize::Solution"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.dataset_group_arn;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'dataset_group_arn'. {} is greater than 256", the_val.len()));
        }

        
        if let Some(the_val) = &self.event_type {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'event_type'. {} is greater than 256", the_val.len()));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.recipe_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'recipe_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        self.solution_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AlgorithmHyperParameterRanges property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlgorithmHyperParameterRanges {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of CategoricalHyperParameterRange
    ///
    /// Update requires: Replacement
    #[serde(rename = "CategoricalHyperParameterRanges")]
    pub categorical_hyper_parameter_ranges: Option<Vec<CategoricalHyperParameterRange>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ContinuousHyperParameterRange
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContinuousHyperParameterRanges")]
    pub continuous_hyper_parameter_ranges: Option<Vec<ContinuousHyperParameterRange>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of IntegerHyperParameterRange
    ///
    /// Update requires: Replacement
    #[serde(rename = "IntegerHyperParameterRanges")]
    pub integer_hyper_parameter_ranges: Option<Vec<IntegerHyperParameterRange>>,

}



impl cfn_resources::CfnResource for AlgorithmHyperParameterRanges {
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

/// The AutoMLConfig property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoMLConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RecipeList")]
    pub recipe_list: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for AutoMLConfig {
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

/// The CategoricalHyperParameterRange property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CategoricalHyperParameterRange {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for CategoricalHyperParameterRange {
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

/// The ContinuousHyperParameterRange property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContinuousHyperParameterRange {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxValue")]
    pub max_value: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinValue")]
    pub min_value: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



impl cfn_resources::CfnResource for ContinuousHyperParameterRange {
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

/// The HpoConfig property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HpoConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AlgorithmHyperParameterRanges
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlgorithmHyperParameterRanges")]
    pub algorithm_hyper_parameter_ranges: Option<AlgorithmHyperParameterRanges>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: HpoObjective
    ///
    /// Update requires: Replacement
    #[serde(rename = "HpoObjective")]
    pub hpo_objective: Option<HpoObjective>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: HpoResourceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "HpoResourceConfig")]
    pub hpo_resource_config: Option<HpoResourceConfig>,

}



impl cfn_resources::CfnResource for HpoConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.algorithm_hyper_parameter_ranges.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.hpo_objective.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.hpo_resource_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The HpoObjective property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HpoObjective {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricRegex")]
    pub metric_regex: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}



impl cfn_resources::CfnResource for HpoObjective {
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

/// The HpoResourceConfig property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HpoResourceConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxNumberOfTrainingJobs")]
    pub max_number_of_training_jobs: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxParallelTrainingJobs")]
    pub max_parallel_training_jobs: Option<String>,

}



impl cfn_resources::CfnResource for HpoResourceConfig {
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

/// The IntegerHyperParameterRange property type specifies Property description not available. for an AWS::Personalize::Solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IntegerHyperParameterRange {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxValue")]
    pub max_value: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinValue")]
    pub min_value: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



impl cfn_resources::CfnResource for IntegerHyperParameterRange {
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

/// Describes the configuration properties for the solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SolutionConfig {


    /// 
    /// Lists the hyperparameter names and ranges.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlgorithmHyperParameters")]
    pub algorithm_hyper_parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The AutoMLConfig object containing a list of recipes to search    when AutoML is performed.
    /// 
    /// Required: No
    ///
    /// Type: AutoMLConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoMLConfig")]
    pub auto_mlconfig: Option<AutoMLConfig>,


    /// 
    /// Only events with a value greater than or equal to this threshold are    used for training a model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventValueThreshold")]
    pub event_value_threshold: Option<String>,


    /// 
    /// Lists the feature transformation parameters.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureTransformationParameters")]
    pub feature_transformation_parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Describes the properties for hyperparameter optimization (HPO).
    /// 
    /// Required: No
    ///
    /// Type: HpoConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "HpoConfig")]
    pub hpo_config: Option<HpoConfig>,

}



impl cfn_resources::CfnResource for SolutionConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.auto_mlconfig.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.event_value_threshold {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'event_value_threshold'. {} is greater than 256", the_val.len()));
        }

        }
        
        self.hpo_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}