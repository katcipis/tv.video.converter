build:
	cargo build

install: build
	cargo install --path .

install-deps:
	sudo pacman -S gstreamer gst-plugins-good gst-plugins-ugly gstreamer-vaapi gst-plugins-base gst-plugins-bad
