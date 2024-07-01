use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub stream_name: String,
    pub stream_id: String,
    pub flows: Vec<Flow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Flow {
    pub name: String,
    pub flow_dependencies: Option<Vec<String>>,
    pub flow_id: String,
    #[serde(rename = "executorID")]
    pub executor_id: String,
    pub process: Vec<Process>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub process_name: String,
    pub process_id: String,
    pub process_binary: String,
    pub process_arguments: Vec<String>,
    pub process_dependencies: Vec<String>,
    pub process_report: String,
}
