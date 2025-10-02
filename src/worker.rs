/*
 * Copyright (C) 2025 Nicolás Ortega Froysa <nicolas@ortegas.org> All rights reserved.
 * Author: Nicolás Ortega Froysa <nicolas@ortegas.org>
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 *
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 *
 * 3. This notice may not be removed or altered from any source
 *    distribution.
 */

use std::io;
use std::fs;
use std::io::Read;
use std::boxed::Box;

pub fn work(in_path:Option<&str>, out_path:Option<&str>) -> Result<(), String>  {
	let in_reader:Box<dyn io::Read> = match in_path {
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

	let mut last_ch = '\0';
	let mut tab_num = 0;

	for i in in_reader.bytes() {
		let ch = i.unwrap() as char;
		match ch {
			'[' | '{' => {
				tab_num += 1;
				out_writer.write(format!("{ch}\n").as_bytes()).unwrap();
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
				out_writer.write(format!("{ch}").as_bytes()).unwrap();
				last_ch = ch;
			},
			',' => {
				out_writer.write(format!("{ch}").as_bytes()).unwrap();
				if last_ch == '}' || last_ch == ']' {
					out_writer.write(b"\n").unwrap();
					for _ in 0..tab_num {
						out_writer.write(b"  ").unwrap();
					}
					last_ch = ' ';
				} else {
					last_ch = ch;
				}
			},
			_ => {
				if ch != ' ' || last_ch != ' ' {
					out_writer.write(format!("{ch}").as_bytes()).unwrap();
					last_ch = ch;
				}
			},
		}
	}

	Ok(())
}
