install:
	sudo cp tv-video-converter.sh /usr/local/bin/tv-video-converter
	sudo cp tv-video-batch-converter.py /usr/local/bin/tv-video-batch-converter
	sudo chmod +x /usr/local/bin/tv-video-converter
	sudo chmod +x /usr/local/bin/tv-video-batch-converter

install-deps:
	sudo pacman -S gstreamer gst-plugins-good gst-plugins-ugly gstreamer-vaapi gst-plugins-base gst-plugins-bad
