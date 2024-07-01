// StreamDef Struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StreamDef {
    pub streamName: String,
    pub streamId: String,
    pub flows: Vec<FlowDef>,
}

// FlowDef Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlowDef {
    pub name: String,
    pub flowId: String,
    pub flowDependencies: Vec<String>,
    pub executorID: String,
    pub process: Vec<ProcDef>,
}

// ProcDef struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcDef {
    pub processName: String,
    pub processId: String,
    pub processBinary: String,
    pub processArguments: Vec<String>,
    pub processDependencies: Vec<String>,
    pub processReport: String,
}
