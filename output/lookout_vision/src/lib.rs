
pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// The name of the Amazon Lookout for Vision project
    #[serde(rename = "ProjectName")]
    pub project_name: ProjectName,

}

pub type ProjectName = String;pub type Arn = String;

}
