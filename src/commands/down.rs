use crate::docker::nephelios_service::NepheliosService;
use bollard::Docker;

pub async fn execute() -> Result<(), anyhow::Error> {
    let docker = Docker::connect_with_local_defaults();
    match docker {
        Ok(docker) => {
            let nephelios_service: NepheliosService = NepheliosService::new(docker, vec![]);
            match nephelios_service.stop().await {
                Ok(_) => {
                    println!("Nephelios stopped successfully");
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Docker: {}", e);
            Err(e.into())
        }
    }
}
