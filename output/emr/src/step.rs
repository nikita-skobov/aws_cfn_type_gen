

/// Use Step to specify a cluster (job flow) step, which runs only on the master node. Steps are used to submit data processing jobs to a cluster.
#[derive(Default, serde::Serialize)]
pub struct CfnStep {


    /// 
    /// This specifies what action to take when the cluster step fails. Possible values are CANCEL_AND_WAIT and CONTINUE.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: String,


    /// 
    /// The name of the cluster step.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A string that uniquely identifies the cluster (job flow).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,


    /// 
    /// The HadoopJarStepConfig property type specifies a job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for the cluster to execute as a step on the master node, and then waits for the job to finish or fail before executing subsequent steps.
    /// 
    /// Required: Yes
    ///
    /// Type: HadoopJarStepConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,

}


/// KeyValue is a subproperty of the HadoopJarStepConfig property type. KeyValue is used to pass parameters to a step.
#[derive(Default, serde::Serialize)]
pub struct KeyValue {


    /// 
    /// The unique identifier of a key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The value part of the identified key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


/// A job flow step consisting of a JAR file whose main function will be executed. The main     function submits a job for Hadoop to execute and waits for the job to finish or     fail.
#[derive(Default, serde::Serialize)]
pub struct HadoopJarStepConfig {


    /// 
    /// A list of command line arguments passed to the JAR file's main function when     executed.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,


    /// 
    /// The name of the main class in the specified Java file. If not specified, the JAR file     should specify a Main-Class in its manifest file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "MainClass")]
    pub main_class: Option<String>,


    /// 
    /// A list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.
    /// 
    /// Required: No
    ///
    /// Type: List of KeyValue
    ///
    /// Update requires: Replacement
    #[serde(rename = "StepProperties")]
    pub step_properties: Option<Vec<KeyValue>>,


    /// 
    /// A path to a JAR file run during the step.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Jar")]
    pub jar: String,

}
