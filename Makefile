f:
	cd frontend; python3 -m http.server

b:
	cargo r -r -j8 --manifest-path ./backend/Cargo.toml
