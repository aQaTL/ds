mod dir_iter;

fn clap_app() -> clap::App<'static, 'static> {
	use clap::*;
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(Arg::with_name("DIR")
			.required(false)
			.index(1)
			.takes_value(true))
}

fn main() {
	let matches = clap_app().get_matches();
	let file = matches
		.value_of("DIR")
		.map_or_else(
			|| std::env::current_dir().expect("Failed to get current dir"),
			|file| std::path::PathBuf::from(file));

	println!("Searched dir: {}", file.display());

	let mut size_sum: u64= 0;

	for file in dir_iter::DirIter::new(&file).expect("Failed to read dir").filter_map(Result::ok) {
		let metadata = match file.metadata() {
			Ok(m) => m,
			Err(e) => {
				eprintln!("Failed to get metadata for {:?} with error: {}", file.path(), e);
				continue;
			},
		};

		if metadata.is_dir() {
			continue;
		}

		size_sum += metadata.len();
	}

	println!("{} B", size_sum);
	if size_sum > 1024 {
		let hr = bytes_in_human_readable(size_sum);
		println!("{:.1} {}B\n{:.1} {}B", hr.bin_value, hr.bin_name, hr.si_value, hr.si_name);
	}
}

struct Multiple<'a> {
	si_name: &'a str, 
	si_value: f32, 

	bin_name: &'a str, 
	bin_value: f32, 
}

fn bytes_in_human_readable(size: u64) -> Multiple<'static> {
	let mut bits = 0u64;

	static MULTIPLES: [(&'static str, &'static str); 8] = [
		("K", "Ki"),
		("M", "Mi"),
		("G", "Gi"),
		("T", "Ti"),
		("P", "Pi"),
		("E", "Ei"),
		("Z", "Zi"),
		("Y", "Yi"),
	];

	while size >> (bits + 10) != 0 {
		bits += 10;
	}

	Multiple {
		si_name: MULTIPLES[(bits / 10 - 1) as usize].0,
	 	si_value: (size as f32 / 10_f32.powi((3 * bits / 10) as i32)),

		bin_name: MULTIPLES[(bits / 10 - 1) as usize].1,
	 	bin_value: (size as f32 / 2_f32.powi(bits as i32)),
	}
}
