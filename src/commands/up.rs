use std::collections::HashMap;

use crate::docker::nephelios_service::NepheliosService;
use anyhow::Result;
use bollard::Docker;

pub async fn execute() -> Result<(), anyhow::Error> {
    let docker = Docker::connect_with_local_defaults();
    match docker {
        Ok(docker) => {
            let nephelios_service: NepheliosService =
                NepheliosService::new(docker, Some("3030/tcp".to_string()), Some(HashMap::new()));

            if !nephelios_service.is_nephelios_running().await {
                if !nephelios_service.is_nephelios_stopped().await {
                    let version: String = "latest".to_string();

                    match nephelios_service.pull_image(version.clone()).await {
                        Ok(_) => println!("Image {} pulled successfully", version),
                        Err(e) => {
                            eprintln!("Failed to pull image {}: {}", version, e);
                            return Err(e.into());
                        }
                    }

                    match nephelios_service.ensure_volumes().await {
                        Ok(_) => println!("Volumes checked/created successfully"),
                        Err(e) => {
                            eprintln!("Failed to check/create volumes: {}", e);
                            return Err(e.into());
                        }
                    }

                    match nephelios_service.create(version.clone()).await {
                        Ok(_) => println!("Nephelios started successfully"),
                        Err(e) => {
                            eprintln!("Failed to start Nephelios: {}", e);
                            return Err(e.into());
                        }
                    }
                }

                println!("Nephelios is stopped, starting it up");

                match nephelios_service.start().await {
                    Ok(_) => println!("Nephelios started successfully"),
                    Err(e) => {
                        eprintln!("Failed to start Nephelios: {}", e);
                        return Err(e.into());
                    }
                }
            } else {
                println!("Nephelios is already running");
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Docker: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
