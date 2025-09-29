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

use std::env;
use std::io;
use std::process;

use crate::worker::work;

mod worker;

fn print_version() {
	println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
	println!("Usage: {} <in-file> [out-file]", env!("CARGO_PKG_NAME"));
}

fn print_help() {
	print_version();
	println!("Usage: \n\
		\t{} <in-file> [out-file]\n\
		\t{} -h | -v\n\
		\n\
		OPTIONS:\n\
		\tin-file   Input file ('-' for stdin)\n\
		\tout-file  Output file\n\
		\t-h        Show this help information\n\
		\t-v        Show version information",
		env!("CARGO_PKG_NAME"), env!("CARGO_PKG_NAME"));
}

fn main() -> io::Result<()> {
	let args:Vec<String> = env::args().collect();
	let mut in_path:Option<&str> = None;
	let mut out_path:Option<&str> = None;

	if args.len() < 2 || args.len() > 3 {
		eprintln!("Invalid number of arguments. Use -h for more information.");
		print_usage();
		process::exit(1);
	}

	for i in &args {
		if i == "-h" {
			print_help();
			return Ok(());
		} else if i == "-v" {
			print_version();
			return Ok(());
		} else if i.starts_with("-") && i != "-" {
			eprintln!("Invalid argument '{}'. Use -h for help information.", i);
			process::exit(1);
		}
	}

	if args[1] != "-" {
		in_path = Some(args[1].as_str());
	}
	if args.len() == 3 {
		out_path = Some(args[2].as_str());
	}

	work(in_path, out_path)
}
