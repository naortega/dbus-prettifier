use std::io;
use std::fs;
use std::io::Read;
use std::boxed::Box;

// Read 1KiB of the file at a time.
static BLOCK_SIZE:usize = 1024;

pub fn work(in_path:Option<&str>, out_path:Option<&str>) -> io::Result<()>  {
	let mut in_reader:Box<dyn io::Read> = match in_path {
		Some(x) => {
			let in_file = fs::File::open(x)?;
			Box::new(io::BufReader::new(in_file))
		},
		None => {
			Box::new(io::stdin())
		},
	};

	let mut out_writer:Box<dyn io::Write> = match out_path {
		Some(x) => {
			let out_file = fs::File::create(x)?;
			Box::new(io::BufWriter::new(out_file))
		},
		None => {
			Box::new(io::stdout())
		},
	};

	let mut read_buf:[u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
	let mut last_ch:char = '\0';
	let mut tab_num = 0;

	while in_reader.read(&mut read_buf)? > 0 {
		for i in read_buf {
			let ch = i as char;
			match ch {
				'[' | '{' => {
					tab_num += 1;
					out_writer.write(format!("{}\n", ch).as_bytes())?;
					for _ in 0..tab_num {
						out_writer.write(b"  ")?;
					}
					last_ch = ' ';
				},
				']' | '}' => {
					tab_num -= 1;
					out_writer.write(b"\n")?;
					for _ in 0..tab_num {
						out_writer.write(b"  ")?;
					}
					out_writer.write(format!("{}", ch).as_bytes())?;
					last_ch = ch;
				},
				',' => {
					out_writer.write(format!("{}\n", ch).as_bytes())?;
					for _ in 0..tab_num {
						out_writer.write(b"  ")?;
					}
					last_ch = ' ';
				},
				_ => {
					if ch != ' ' || last_ch != ' ' {
						out_writer.write(format!("{}", ch).as_bytes())?;
						last_ch = ch;
					}
				},
			}
		}
	}

	out_writer.flush()?;

	Ok(())

}
