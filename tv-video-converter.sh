#!/bin/sh

echo ""
echo "It will transcode videos to a more usual TV friendly format and also overlay subtitles."
echo "The format will be a mp4 container with H.264 for video and mp3 for sound"
echo "usage: $0 <original video filepath> <subtitle filepath> <converted video filepath>"
echo ""

echo "Video filepath: "$1" Subtitle filepath: "$2" Converted video filepath: "$3

gst-launch-1.0 filesrc location=$1 ! decodebin name=decoder ! queue ! videoconvert ! videoscale ! video/x-raw,width=1920,height=1080 ! subtitleoverlay font-desc="DejaVu Sans 25px" name=subtitle ! x264enc ! mp4mux name=muxer ! filesink location=$3 decoder. ! audioconvert ! audioresample ! queue ! lamemp3enc ! muxer. filesrc location=$2 ! queue ! subparse ! subtitle.subtitle_sink
