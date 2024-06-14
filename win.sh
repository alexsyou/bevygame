#!/bin/sh
cargo build --release --target x86_64-pc-windows-msvc &&
	cp target/x86_64-pc-windows-msvc/release/bevygame.exe . &&
	exec ./bevygame.exe "$@"
