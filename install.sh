#!/bin/bash

cargo build --release && \
	cp ./target/release/rp ~/.local/bin/rp && \
	chmod +x ~/.local/bin/rp && \
	echo Done.


