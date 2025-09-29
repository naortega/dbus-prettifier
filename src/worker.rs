use std::io;
use std::fs;
use std::io::Read;
use std::boxed::Box;

// Read 1KiB of the file at a time.
static BLOCK_SIZE:usize = 1024;

pub fn work(in_path:Option<&str>, out_path:Option<&str>) -> Result<(), String>  {
	let mut in_reader:Box<dyn io::Read> = match in_path {
		Some(path) => {
			let in_file = fs::File::open(path);
			if let Err(e) = in_file {
				return Err(format!("{path}: {e}"));
			}
			Box::new(io::BufReader::new(in_file.unwrap()))
		},
		None => {
			Box::new(io::stdin())
		},
	};

	let mut out_writer:Box<dyn io::Write> = match out_path {
		Some(path) => {
			let out_file = fs::File::create(path);
			if let Err(e) = out_file {
				return Err(format!("{path}: {e}"));
			}
			Box::new(io::BufWriter::new(out_file.unwrap()))
		},
		None => {
			Box::new(io::stdout())
		},
	};

	let mut read_buf:[u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
	let mut last_ch:char = '\0';
	let mut tab_num = 0;

	while in_reader.read(&mut read_buf).unwrap() > 0 {
		for i in read_buf {
			let ch = i as char;
			match ch {
				'[' | '{' => {
					tab_num += 1;
					out_writer.write(format!("{}\n", ch).as_bytes()).unwrap();
					for _ in 0..tab_num {
						out_writer.write(b"  ").unwrap();
					}
					last_ch = ' ';
				},
				']' | '}' => {
					tab_num -= 1;
					out_writer.write(b"\n").unwrap();
					for _ in 0..tab_num {
						out_writer.write(b"  ").unwrap();
					}
					out_writer.write(format!("{}", ch).as_bytes()).unwrap();
					last_ch = ch;
				},
				',' => {
					out_writer.write(format!("{}\n", ch).as_bytes()).unwrap();
					for _ in 0..tab_num {
						out_writer.write(b"  ").unwrap();
					}
					last_ch = ' ';
				},
				_ => {
					if ch != ' ' || last_ch != ' ' {
						out_writer.write(format!("{}", ch).as_bytes()).unwrap();
						last_ch = ch;
					}
				},
			}
		}
	}

	out_writer.flush().unwrap();

	Ok(())
}
