use std::thread;
use qr_generator::{generate_metadata_qr, config::read_config};


fn main() -> Result<(), String> {
	let config = match read_config("config.toml") {
		Ok(x) => x,
		Err(e) => return Err(format!("Error reading config file. {}", e)),
	};

	println!("{:?}", config);
	let mut handlers = vec![];
	for chain in config.chains {
		let handle = thread::spawn(move || {
			if let Err(e) = generate_metadata_qr(&chain) {
				eprintln!("Error generating QR for {}: {}", chain.name, e)
			}
		});
		handlers.push(handle);
	}

	while let Some(handle) = handlers.pop() {
		handle.join().unwrap();
	}
	Ok(())
}
