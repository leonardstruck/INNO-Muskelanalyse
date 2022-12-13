import os
import cv2
import matplotlib
import numpy
import math
import sys


def getParams(img):
    height, width, channels = img.shape  # BGR

    # Arrays with furthest points
    topLeft = [height, width]
    topRight = [height, 0]
    bottomLeft = [0, width]
    bottomRight = [0, 0]

    for i in range(0, height):
        for j in range(0, width):
            if img[i, j, 1] > 0 and img[i, j, 2] > 0 and img[i, j, 0] > 0:
                if i <= topLeft[0] and j <= topLeft[1]:
                    topLeft = [i, j]
                if i <= topRight[0] and j >= topRight[1]:
                    topRight = [i, j]
                if i >= bottomRight[0] and j >= bottomRight[1]:
                    bottomRight = [i, j]
                if i >= bottomRight[0]:
                    if i > bottomRight[0]:
                        bottomRight = [i, j]
                    elif j > bottomRight[1]:
                        bottomRight = [i, j]
                if i > bottomLeft[0]:
                    bottomLeft = [i, j]
    midTop = [float((topLeft[0] + topRight[0]) / 2),
              float((topLeft[1] + topRight[1]) / 2)]
    midBottom = [float((bottomLeft[0] + bottomRight[0]) / 2),
                 float((bottomLeft[1] + bottomRight[1]) / 2)]
    direction = math.atan(
        (midTop[1] - midBottom[1])/(midTop[0] - midBottom[0])) * 180 / math.pi
    print("Direction: ", direction)
    length = numpy.sqrt(numpy.square(
        midTop[0] - midBottom[0]) + numpy.square(midTop[1] - midBottom[1]))
    print("Length: ", length)  # +/- 3 Pixel bis jetzt bei meinen 2 Tests


def checkFragments():
    directory = "fragments"
    for filename in os.listdir(directory):
        print(filename)
        f = os.path.join(directory, filename)
        # checking if it is a file
        if os.path.isfile(f):
            # libpng warning: iCCP: known incorrect sRGB profile
            img = cv2.imread(f)
            getParams(img)


def main():
    if os.path.isdir("fragments"):
        print("Fragments folder found")
        checkFragments()
    else:
        print("No fragments folder found")
        raise Exception("No fragments folder found")


main()
