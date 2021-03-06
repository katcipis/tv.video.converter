#!/bin/sh

echo ""
echo "It will transcode videos to a more usual TV friendly format and also overlay subtitles."
echo "The format will be a webm container with VP8 for video and Vorbis for sound"
echo ""

echo "usage: $0 <original video filepath> <converted video filepath> <subtitle path (optional)>"
echo ""

input_path=$1
output_path=$2
subtitle_path=$3
muxer=webmmux
videoencoder=vp8enc
audioencoder=vorbisenc

if [ "$#" -eq 2 ]; then

    echo "Video filepath: "$input_path" Converted video filepath: "$output_path

    gst-launch-1.0 filesrc location="$input_path" ! decodebin name=decoder ! queue ! videoconvert ! videoscale ! $videoencoder ! queue ! $muxer name=muxer ! queue ! filesink location="$output_path" decoder. ! queue ! audioconvert ! audioresample ! $audioencoder ! queue ! muxer.

fi

if [ "$#" -eq 3 ]; then

    echo "Video filepath: "$input_path" Converted video filepath: "$output_path" Subtitle path: "$subtitle_path

    gst-launch-1.0 filesrc location="$input_path" ! decodebin name=decoder ! queue ! videoconvert ! videoscale ! subtitleoverlay font-desc="DejaVu Sans 25px" name=subtitle ! $videoencoder ! queue ! $muxer name=muxer ! queue ! filesink location="$output_path" decoder. ! queue ! audioconvert ! audioresample ! $audioencoder ! queue ! muxer. filesrc location="$subtitle_path" ! queue ! subparse ! subtitle.subtitle_sink

fi
