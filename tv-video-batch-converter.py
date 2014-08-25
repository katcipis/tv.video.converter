#! /usr/bin/env python

import sys
import os
import glob

def getSourceDir():
    return os.path.expanduser(sys.argv[1]);

def getFilesWithSuffix (directory, suffix):
    return glob.glob(sourceDir + "/*." + suffix);

def getSourceVideos(sourceDir):
    videos = [];
    subtitleSuffix = "srt";
    videos.extend(getFilesWithSuffix(sourceDir, "mkv"));
    videos.extend(getFilesWithSuffix(sourceDir, "mp4"));
    videos.extend(getFilesWithSuffix(sourceDir, "avi"));
    subtitles = getFilesWithSuffix(sourceDir, subtitleSuffix);
    result = [];

    for video in videos:
        expectedSubtitle = video[:-3] + subtitleSuffix;
        if expectedSubtitle in subtitles:
            result.append({ "video" : video, "subtitle" : expectedSubtitle});

    return result;

def getSinkDir():
    return os.path.expanduser(sys.argv[2]);

def askUserToGoOn():
    pass

def generateSinkVideo(sourceVideo, sinkDir):
    pass

def printDescriptionOfVideos(sourceVideos):
    for videoInfo in sourceVideos:
        print("\nVideo: {video}\nSubtitle: {subtitle}\n".format(**videoInfo));


if len(sys.argv) < 3:
    print("usage: {0[0]} <source dir of videos to transcode> <sink dir where transcoded videos will be saved>".format(sys.argv));
    exit();


sourceDir = getSourceDir();
sourceVideos = getSourceVideos(sourceDir);

sinkDir = getSinkDir();

printDescriptionOfVideos(sourceVideos);
askUserToGoOn();

print("Starting the transcoding process, this may take a while");

for sourceVideo in sourceVideos:
    generateSinkVideo(sourceVideo, sinkDir);

print("Done :-)");
