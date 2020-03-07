use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;

fn run() -> Result<(), Box<std::error::Error>> {
    let allowed_devices = load_devices()?;

    if allowed_devices.is_empty() {
        println!("please add some devices to ~/.config/beryl/devices");
        std::process::exit(1);
    }

    let session = BluetoothSession::create_session(None)?;
    let adapter = BluetoothAdapter::init(&session)?;

    loop {
        println!("listening...");
        let devices = adapter.get_device_list()?;
        for d in devices {
            println!("found {}", &d);

            // Only if our allow strings match anything in this list.
            if !allowed_devices.iter().any(|allow| d.contains(allow)) {
                continue;
            }

            let device = BluetoothDevice::new(&session, d.to_string());
            if !device.is_connected()? {
                println!("connecting ...");
                let attempt = device.connect(5000);
                match attempt {
                    Ok(success) => {
                        println!("success");
                    }
                    Err(err) => {
                        println!("error {:?}", err);
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}

fn load_devices() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut config = dirs::config_dir().ok_or("couldn't get config dir")?;
    config.push("beryl");

    if !config.exists() {
        std::fs::create_dir_all(&config)?;
    }

    config.push("devices");

    if !config.exists() {
        std::fs::File::create(&config)?;
    }

    let mut file = std::fs::File::open(&config)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    Ok(contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect())
}

fn main() {
    run().unwrap();
}
