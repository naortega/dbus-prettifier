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

static MAX_WRITE_BUF_SIZE:usize = 1024;

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
	let mut in_paren = false;
	let mut write_buf = String::new();

	for i in in_reader.bytes() {
		let ch = i.unwrap() as char;
		match ch {
			'[' | '{' => {
				tab_num += 1;
				//write_buf += format!("{ch}\n").as_str();
				write_buf.push(ch);
				if !in_paren {
					write_buf.push('\n');
					for _ in 0..tab_num {
						write_buf += "  ";
					}
					last_ch = ' ';
				} else {
					last_ch = ch;
				}
			},
			']' | '}' => {
				tab_num -= 1;
				if !in_paren {
					write_buf += "\n";
					for _ in 0..tab_num {
						write_buf += "  ";
					}
				}
				write_buf.push(ch);
				last_ch = ch;
			},
			',' => {
				write_buf.push(ch);
				if last_ch == '}' || last_ch == ']' {
					write_buf.push('\n');
					for _ in 0..tab_num {
						write_buf += "  ";
					}
					last_ch = ' ';
				} else {
					last_ch = ch;
				}
			},
			_ => {
				if ch != ' ' || last_ch != ' ' {
					if ch == '(' {
						in_paren = true;
					} else if ch == ')' {
						in_paren = false;
					}
					write_buf.push(ch);
					last_ch = ch;
				}
			},
		}

		if write_buf.len() >= MAX_WRITE_BUF_SIZE {
			out_writer.write_all(write_buf.as_bytes()).unwrap();
			write_buf.clear();
		}
	}

	if write_buf.len() > 0 {
		out_writer.write_all(write_buf.as_bytes()).unwrap();
	}

	Ok(())
}
