use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, StartContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::result::Result::Ok;

use super::volumes::nephelios_volume::NepheliosVolume;

pub struct NepheliosService {
    pub docker: Docker,
    pub name: String,
    pub image: String,
    pub socket: String,
    pub volumes: Vec<NepheliosVolume>,
    pub exposed_port: String,
    pub env: Vec<String>,
}

impl NepheliosService {
    pub fn new(
        docker: Docker,
        exposed_port: Option<String>,
        env: Option<HashMap<String, String>>,
    ) -> Self {
        let port = exposed_port.map_or("3030".to_string(), |v| v.clone());
        Self {
            docker,
            name: "nephelios".to_string(),
            image: "zuhowks/nephelios".to_string(),
            socket: "/var/run/docker.sock".to_string(),
            volumes: {
                let volumes = vec![
                    NepheliosVolume {
                        name: "grafana_data",
                        mount_path: "/var/lib/nephelios/grafana",
                    },
                    NepheliosVolume {
                        name: "grafana_provisioning",
                        mount_path: "/app/config/grafana",
                    },
                    NepheliosVolume {
                        name: "grafana_dashboard",
                        mount_path: "/app/config/dashboards",
                    },
                    NepheliosVolume {
                        name: "prometheus_data",
                        mount_path: "/app/prometheus",
                    },
                    NepheliosVolume {
                        name: "registry_data",
                        mount_path: "/var/lib/nephelios/registry",
                    },
                    NepheliosVolume {
                        name: "nephelios_data",
                        mount_path: "/app/config/prometheus",
                    },
                ];
                volumes
            },
            exposed_port: port.clone(),
            env: {
                let mut neph_env: Vec<String> = vec![];

                if let Some(env) = env {
                    neph_env.push(format!("NEPHELIOS_PORT={}", port));
                    neph_env.push(format!(
                        "NEPHELIOS_APPS_PORT={}",
                        env.get("NEPHELIOS_APPS_PORT").map_or("5173", |v| v)
                    ));
                    neph_env.push(format!(
                        "LEAVE_SWARM={}",
                        env.get("LEAVE_SWARM").map_or("false", |v| v)
                    ));
                    neph_env.push(format!(
                        "ADVERTISE_ADDR={}",
                        env.get("ADVERTISE_ADDR",).map_or("127.0.0.1", |v| v)
                    ));
                }

                neph_env
            },
        }
    }
    pub fn get_label(&self) -> String {
        format!("com.nephelios.name={}", self.name)
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        let option = StartContainerOptions {
            detach_keys: "ctrl-d",
            ..Default::default()
        };

        let start_stream = self
            .docker
            .start_container(self.name.as_str(), Some(option));

        start_stream
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start container: {}", e))
    }

    pub async fn stop(&self) -> Result<(), anyhow::Error> {
        self.docker
            .stop_container(self.name.as_str(), None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to stop container: {}", e))
    }

    async fn check_nephelios(&self, filters: HashMap<&str, Vec<&str>>) -> bool {
        let options = Some(ListContainersOptions {
            filters,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await;
        match containers {
            Ok(containers) => !containers.is_empty(),
            Err(e) => {
                eprintln!("Failed to list containers: {}", e);
                false
            }
        }
    }

    pub async fn is_nephelios_running(&self) -> bool {
        let label = self.get_label();
        let mut filters: HashMap<&str, Vec<&str>> = HashMap::new();
        filters.insert("status", vec!["running"]);
        filters.insert("label", vec![label.as_str()]);
        self.check_nephelios(filters).await
    }

    pub async fn is_nephelios_stopped(&self) -> bool {
        let label = self.get_label();
        let mut filters: HashMap<&str, Vec<&str>> = HashMap::new();
        filters.insert("status", vec!["exited"]);
        filters.insert("label", vec![label.as_str()]);
        self.check_nephelios(filters).await
    }

    pub async fn pull_image(&self, version: String) -> Result<(), anyhow::Error> {
        let image = format!("{}:{}", self.image, version);

        let options = Some(CreateImageOptions {
            from_image: image,
            ..Default::default()
        });

        let mut create_stream = self.docker.create_image(options, None, None);

        while let Some(output) = create_stream.next().await {
            match output {
                Ok(output) => {
                    if let Some(stream) = output.progress {
                        match serde_json::from_str::<serde_json::Value>(&stream) {
                            Ok(value) => {
                                if let Some(status) = value.get("status") {
                                    println!("Pull Image info: {}", status);
                                }
                            }
                            Err(_) => {
                                println!("Pull Image info: {}", stream);
                            }
                        }
                    }
                    if let Some(error) = output.error {
                        eprintln!("Error: {}", error);
                    }
                }
                Err(e) => {
                    eprintln!("Error pullings image: {}", e);
                }
            }
        }
        Ok(())
    }

    pub async fn create(&self, version: String) -> Result<(), anyhow::Error> {
        let label = self.get_label();

        let options = Some(CreateContainerOptions {
            name: self.name.clone(),
            platform: None,
        });

        let config = Config {
            image: Some(format!("{}:{}", self.image, version)),
            host_config: Some(bollard::service::HostConfig {
                binds: Some({
                    let mut binds = vec![];
                    binds.push(format!("{}:/var/run/docker.sock", self.socket));
                    for volume in &self.volumes {
                        binds.push(format!("{}:{}", volume.name, volume.mount_path));
                    }

                    binds
                }),
                ..Default::default()
            }),
            labels: Some({
                let mut labels = HashMap::new();

                labels.insert("com.nephelios.name".to_string(), self.name.clone());
                labels.insert(label.clone(), label);
                labels
            }),
            env: Some(self.env.clone()),
            exposed_ports: Some(HashMap::from([(self.exposed_port.clone(), HashMap::new())])),
            ..Default::default()
        };

        let result: Result<bollard::secret::ContainerCreateResponse, bollard::errors::Error> =
            self.docker.create_container(options, config).await;

        match result {
            Ok(res) => {
                println!("Container {} created successfully", res.id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to create container: {}", e);
                Err(e.into())
            }
        }
    }

    /// Ensures that all required Docker volumes exist for Nephelios.
    /// If any volume doesn't exist, it will be created.
    ///
    /// # Returns
    /// * `Ok(())` if all volumes were successfully checked/created
    /// * `Err(String)` if there was an error during the process
    pub async fn ensure_volumes(&self) -> Result<(), anyhow::Error> {
        for volume in self.volumes.iter() {
            if !(volume.is_volume_created(&self.docker).await) {
                if !volume.create_volume(&self.docker).await {
                    return Err(anyhow::anyhow!("Failed to create volume {}", volume.name));
                }
            }
        }

        Ok(())
    }
}
