#[derive(Clone, Debug)]
pub struct DataLogContext {
    pub project_uuid: String,
    pub datasync_uuid: String,
    pub volume_uuid: String,
    pub worker_id: String,
}

impl Default for DataLogContext {
    fn default() -> Self {
        Self {
            project_uuid: "null".into(),
            datasync_uuid: "null".into(),
            volume_uuid: "null".into(),
            worker_id: "null".into(),
        }
    }
}

impl DataLogContext {
    pub fn with_project_uuid(mut self, project_uuid: String) -> Self {
        self.project_uuid = project_uuid;
        self
    }
    pub fn with_datasync_uuid(mut self, datasync_uuid: String) -> Self {
        self.datasync_uuid = datasync_uuid;
        self
    }
    pub fn with_volume_uuid(mut self, progress_uuid: String) -> Self {
        self.volume_uuid = progress_uuid;
        self
    }
    pub fn with_worker_id(mut self, worker_id: u32) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }
}
