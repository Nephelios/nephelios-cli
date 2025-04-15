use bollard::{volume::CreateVolumeOptions, Docker};
use std::collections::HashMap;

pub struct NepheliosVolume {
    pub name: &'static str,
    pub mount_path: &'static str,
}

impl NepheliosVolume {
    pub fn new(name: &'static str, mount_path: &'static str) -> Self {
        Self { name, mount_path }
    }

    pub async fn is_volume_created(&self, docker: &Docker) -> bool {
        // Check if volume exists
        let filters: HashMap<String, Vec<String>> = {
            let mut map = HashMap::new();
            map.insert("name".to_string(), vec![self.name.to_string()]);
            map
        };

        let options = bollard::volume::ListVolumesOptions { filters };

        let volumes = docker.list_volumes(Some(options)).await;

        match volumes {
            Ok(volumes) => volumes.volumes.is_some(),
            Err(e) => {
                eprintln!("Failed to list volumes: {}", e);
                false
            }
        }
    }

    pub async fn create_volume(&self, docker: &Docker) -> bool {
        let option = CreateVolumeOptions {
            name: self.name.to_string(),

            ..Default::default()
        };

        // Create volume
        let result = docker.create_volume(option).await;
        match result {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to create volume {}: {}", self.name, e);
                false
            }
        }
    }
}
