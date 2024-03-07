//! Docker commands wrapper with default options specifically for running deep learning containers.
use anyhow::Result;
use clap::Parser;
use std::{
    env,
    process::{exit, Command, Stdio},
};
/// Struct representing the options for running a Docker container.
#[derive(Debug, Parser)]
struct DockerOptions {
    /// The image and tag of the Docker container.
    image_and_tag: String,
    /// Whether to remove the container after it exits.
    #[arg(long)]
    rm: bool,
    /// The name of the container.
    #[arg(long)]
    name: String,
    /// The network to connect the container to. Defaults to "host".
    #[arg(default_value = "host")]
    net: Option<String>,
    /// The GPUs to assign to the container. Defaults to "all".
    #[arg(default_value = "all")]
    gpus: Option<String>,
    /// The volumes to mount in the container. Defaults to "~/workspace:/root/workspace".
    #[arg(default_value = "~/workspace:/root/workspace")]
    volumes: Option<String>,
}

fn parse_volume_str(volume_string: &str) -> Result<String> {
    let home_dir = env::var("HOME")?;
    let mut volume_string = volume_string.to_string();
    volume_string = volume_string.replace("~", &home_dir);
    Ok(volume_string)
}

fn main() -> Result<()> {
    // Parse the command line arguments into a `DockerOptions` struct.
    let args = DockerOptions::parse();

    // Create a new `Command` to run the `docker` command.
    let mut command = Command::new("docker");

    // Add the initial arguments to the `docker` command.
    command
        .arg("run")
        .arg("-it")
        .arg("-d")
        .arg("--name")
        .arg(args.name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Check if the `rm` flag is set and add the `--rm` argument if it is.
    if args.rm {
        command.arg("--rm");
    }

    // Check if the `gpus` option is set and add the `--gpus` argument with the specified value.
    if let Some(gpus) = args.gpus {
        command.arg("--gpus").arg(gpus);
    }

    // Add the network argument with the specified value.
    if let Some(net) = args.net {
        command.arg("--net").arg(net);
    }

    // Add the volumes argument with the specified value.
    if let Some(volumes) = args.volumes {
        let volumes = parse_volume_str(&volumes)?;
        command.arg("-v").arg(volumes);
    }

    // Add the image and tag.
    command.arg(args.image_and_tag);

    // Print the command that will be executed.
    println!("Running command: {:?}", command);

    // Execute the command and capture the output.
    let output = command.output().expect("Failed to run the command");

    exit(output.status.code().unwrap_or(1))
}
