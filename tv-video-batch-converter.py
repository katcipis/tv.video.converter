#! /usr/bin/env python

def getSourceDir():
    pass

def getSourceVideos(sourceDir):
    return [];

def getSinkDir():
    pass

def askUserToGoOn():
    pass

def generateSinkVideo(sourceVideo, sinkDir):
    pass

def printDescriptionOfVideos(sourceVideos):
    pass


sourceDir = getSourceDir();
sourceVideos = getSourceVideos(sourceDir);

sinkDir = getSinkDir();

printDescriptionOfVideos(sourceVideos);
askUserToGoOn();

print("Starting the transcoding process, this may take a while");

for sourceVideo in sourceVideos:
    generateSinkVideo(sourceVideo, sinkDir);

print("Done :-)");
