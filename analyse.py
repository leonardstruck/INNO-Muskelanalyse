import os
import cv2
import matplotlib
import numpy
import math
import sys


def getMidPoint(p1, p2):
    return [float((p1[0] + p2[0]) / 2), float((p1[1] + p2[1]) / 2)]


def calcThickness(img, direction, middle, length):
    # direction in +/- 90° weiter gehen bis wir auf ein schwarzes pixel treffen
    # a^2 + b^2 = c^2
    a = float(length/2)
    b = float(length/2)
    c = float(numpy.sqrt(numpy.square(a) + numpy.square(b)))


def calcPoints(img):
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
    midMiddle = getMidPoint(midTop, midBottom)
    print("MidTop: ", midTop)
    print("MidBottom: ", midBottom)
    print("MidMiddle: ", midMiddle)
    return midTop, midBottom, midMiddle


def getParams(img):
    # diese Funktion muss noch verbessert werden -> genauer werden
    top, bottom, middle = calcPoints(img)

    direction = ((math.atan(
        (top[1] - bottom[1])/(top[0] - bottom[0])) * 180 / math.pi) + 180) % 360  # + 180 um 0 grad anders zu definieren
    print("Direction: ", direction)
    length = numpy.sqrt(numpy.square(
        top[0] - bottom[0]) + numpy.square(top[1] - bottom[1]))
    print("Length: ", length)  # +/- 3 Pixel bis jetzt bei meinen 2 Tests

    thickness = calcThickness(img, direction, middle, length)

    start = (int(top[1]), int(top[0]))
    end = (int(bottom[1]), int(bottom[0]))

    # show line
    cv2.line(img, start, end, (255, 0, 255), 1)
    cv2.imshow("img", img)
    cv2.waitKey(0)


def checkFragments():
    directory = "fragments"
    for filename in os.listdir(directory):
        print()
        print(filename)
        f = os.path.join(directory, filename)
        # checking if it is a file
        img = cv2.imread(f)
        if os.path.isfile(f) and img is not None:
            # libpng warning: iCCP: known incorrect sRGB profile
            getParams(img)


def main():
    if os.path.isdir("fragments"):
        print("Fragments folder found")
        checkFragments()
    else:
        print("No fragments folder found")
        raise Exception("No fragments folder found")


main()
