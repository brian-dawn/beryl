use std::io::prelude::*;
use std::process::Command;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;

use serde::Deserialize;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_devices()?;
    println!("{:?}", config);

    if config.devices.is_empty() {
        println!("please add some devices to ~/.config/beryl/config.toml");
        std::process::exit(1);
    }

    let session = BluetoothSession::create_session(None)?;
    let adapter = BluetoothAdapter::init(&session)?;

    loop {
        println!("listening...");
        let devices = adapter.get_device_list()?;
        for d in devices {
            println!("found {}", &d);

            let maybe_config_device = config
                .devices
                .iter()
                .find(|device| d.contains(device.id.trim()));
            if let Some(config_device) = maybe_config_device {
                let device = BluetoothDevice::new(&session, d.to_string());
                if !device.is_connected()? {
                    println!("connecting ...");
                    let attempt = device.connect(5000);
                    match attempt {
                        Ok(_) => {
                            println!("success");
                            // Run the command if we have one.
                            if let Some(command) = &config_device.command {
                                println!("running {}", command);
                                
                                let parts:Vec<String> = command.split_whitespace().map(|i| i.to_owned()).collect();
                                if parts.is_empty() {
                                    println!("invalid command");
                                    continue;
                                }
                                let mut command = Command::new(parts[0].clone());
                                for part in parts.iter().skip(1) {
                                    command.arg(part);
                                }
                                command.output()?;


                                  
                            }
                        }
                        Err(err) => {
                            println!("error {:?}", err);
                        }
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}

#[derive(Debug, Deserialize)]
struct Device {
    id: String,
    command: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    devices: Vec<Device>,
}

fn load_devices() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = dirs::config_dir().ok_or("couldn't get config dir")?;
    config.push("beryl");

    if !config.exists() {
        std::fs::create_dir_all(&config)?;
    }

    config.push("config.toml");

    if !config.exists() {
        std::fs::File::create(&config)?;
    }

    let mut file = std::fs::File::open(&config)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str(&contents)?)
}

fn main() {
    run().unwrap();
}
