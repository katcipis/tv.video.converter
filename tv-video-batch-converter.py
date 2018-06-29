#! /usr/bin/env python2

import sys
import os
import glob
import subprocess


def getSourceDir():
    return os.path.expanduser(sys.argv[1]);


def getFilesWithSuffix (directory, suffix):
    return glob.glob(sourceDir + "/*." + suffix);


def getSourceVideos(sourceDir):
    videos = [];
    videos.extend(getFilesWithSuffix(sourceDir, "mkv"));
    videos.extend(getFilesWithSuffix(sourceDir, "mp4"));
    videos.extend(getFilesWithSuffix(sourceDir, "avi"));
    result = [];

    for video in videos:
        expectedSubtitle = video[:-3] + "srt";
        if os.path.exists(expectedSubtitle):
            result.append({ "video" : video, "subtitle" : expectedSubtitle});
        else:
        	result.append({ "video" : video });

    return result;


def getSinkDir():
    return os.path.expanduser(sys.argv[2]);


def askUserToGoOn():
    proceed = raw_input("Do you want to go on ? this will probably take a lot of time (y/N): ");
    if not 'y' in proceed.lower():
        print("User requested to abort");
        exit();
    print("Going on to process a bunch of videos");


def generateSinkVideo(sourceVideo, sinkDir):
    args = ["tv-video-converter"];
    sinkVideo = os.path.basename(sourceVideo["video"])[:-3] + "mp4";
    args.append(sourceVideo["video"]);
    args.append(os.path.join(sinkDir, sinkVideo));
    
    if "subtitle" in sourceVideo:
    	args.append(sourceVideo["subtitle"]);
    	
    subprocess.call(args);


def printDescriptionOfVideos(sourceVideos):
    print("\n\nThe following videos have been detected: \n");
    for videoInfo in sourceVideos:
    	if "subtitle" in videoInfo:
        	print("\nVideo: {video}\nSubtitle: {subtitle}\n".format(**videoInfo));
        else:
        	print("\nVideo: {video}".format(**videoInfo))
    print("Total of videos to be transcoded: " + str(len(sourceVideos)) + "\n")


if len(sys.argv) < 3:
    print("usage: {0[0]} <source dir of videos to transcode> <sink dir where transcoded videos will be saved>".format(sys.argv));
    exit();


sourceDir = getSourceDir();
sourceVideos = getSourceVideos(sourceDir);
if not sourceVideos:
    print("No video found at sourceDir: [{0}]".format(sourceDir));
    exit();

sinkDir = getSinkDir();

printDescriptionOfVideos(sourceVideos);
askUserToGoOn();

print("Starting the transcoding process, this may take a while. All videos will be saved on: {0}".format(sinkDir));

for sourceVideo in sourceVideos:
    generateSinkVideo(sourceVideo, sinkDir);

print("Done :-)");
