r:
	cargo r --release -- "5 * 6 * 7 * 8"
load:
	cargo r --release -- --load "input.txt"

test: r load
