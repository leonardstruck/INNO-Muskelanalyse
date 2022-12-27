import imutils
import argparse
# import numpy as np
from imutils import contours
from imutils import perspective
from scipy.spatial import distance as dist
import os
import cv2
import matplotlib
import numpy
import math
import sys


def getMidPoint(p1, p2):
    return [float((p1[0] + p2[0]) / 2), float((p1[1] + p2[1]) / 2)]


def getDirection(p1, p2):
    #print("p1: ", p1)
    #print("p2: ", p2)
    if p1[0] == p2[0]:
        return 90
    direction = math.atan((p1[1] - p2[1])/(p1[0] - p2[0])) * 180 / math.pi
    if direction < 0:
        direction += 180
    return direction


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

    start = (int(top[1]), int(top[0]))
    end = (int(bottom[1]), int(bottom[0]))

    # show line
    lined = img.copy()
    cv2.line(lined, start, end, (255, 0, 255), 1)
    cv2.imshow("img", lined)
    cv2.waitKey(0)


def midpoint(ptA, ptB):
    return ((ptA[0] + ptB[0]) * 0.5, (ptA[1] + ptB[1]) * 0.5)


def boxTest(image):
    # quelle: https://pyimagesearch.com/2016/03/28/measuring-size-of-objects-in-an-image-with-opencv/

    # image = cv2.imread("fragments/LB17.png")
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    edged = cv2.Canny(gray, 50, 100)
    # close gaps between object edges
    edged = cv2.dilate(edged, None, iterations=1)
    edged = cv2.erode(edged, None, iterations=1)
    # konturen finden
    cnts = cv2.findContours(edged.copy(), cv2.RETR_EXTERNAL,
                            cv2.CHAIN_APPROX_SIMPLE)
    cnts = imutils.grab_contours(cnts)
    # print("contours: ", cnts)

    (cnts, _) = contours.sort_contours(cnts)
    pixelsPerMetric = None
    cv2.imshow("edged", edged)
    cv2.waitKey(0)

    for c in cnts:
        # if the contour is not sufficiently large, ignore it
        if cv2.contourArea(c) < 1:
            continue
        # compute the rotated bounding box of the contour
        orig = image.copy()
        box = cv2.minAreaRect(c)
        box = cv2.cv.BoxPoints(box) if imutils.is_cv2() else cv2.boxPoints(box)
        box = numpy.array(box, dtype="int")
        # order the points in the contour such that they appear
        # in top-left, top-right, bottom-right, and bottom-left
        # order, then draw the outline of the rotated bounding
        # box
        box = perspective.order_points(box)
        cv2.drawContours(orig, [box.astype("int")], -1, (0, 255, 0), 1)
        # loop over the original points and draw them
        for (x, y) in box:
            cv2.circle(orig, (int(x), int(y)), 5, (0, 0, 255), -1)
        # unpack the ordered bounding box, then compute the midpoint
        # between the top-left and top-right coordinates, followed by
        # the midpoint between bottom-left and bottom-right coordinates
        (tl, tr, br, bl) = box
        (tltrX, tltrY) = midpoint(tl, tr)
        (blbrX, blbrY) = midpoint(bl, br)
        # compute the midpoint between the top-left and top-right points,
        # followed by the midpoint between the top-righ and bottom-right
        (tlblX, tlblY) = midpoint(tl, bl)
        (trbrX, trbrY) = midpoint(tr, br)
        # draw lines between the midpoints
        cv2.line(orig, (int(tltrX), int(tltrY)), (int(blbrX), int(blbrY)),
                 (255, 0, 255), 1)
        cv2.line(orig, (int(tlblX), int(tlblY)), (int(trbrX), int(trbrY)),
                 (255, 0, 255), 1)
        cv2.imshow("Image", orig)
        cv2.waitKey(0)
        # compute the Euclidean distance between the midpoints
        dA = dist.euclidean((tltrX, tltrY), (blbrX, blbrY))
        dB = dist.euclidean((tlblX, tlblY), (trbrX, trbrY))
        if dA > dB:
            direction = getDirection((tltrX, tltrY), (blbrX, blbrY))
        else:
            direction = getDirection((tlblX, tlblY), (trbrX, trbrY))
        # if the pixels per metric has not been initialized, then
        # compute it as the ratio of pixels to supplied metric
        # (in this case, inches)
        if pixelsPerMetric is None:
            pixelsPerMetric = dB / image.shape[1]
        print("dA: ", round(dA, 2))
        print("dB: ", round(dB, 2))
        print("direction: {}°".format(round(direction, 2)))


def checkFragmentsFromFolder():
    directory = "fragments"
    for filename in os.listdir(directory):
        print()
        print(filename)
        f = os.path.join(directory, filename)
        # checking if it is a file
        img = cv2.imread(f)
        if os.path.isfile(f) and img is not None:
            # libpng warning: iCCP: known incorrect sRGB profile
            print("boxTest: ")
            boxTest(img)
            # print("getParams: ")
            # getParams(img)


def checkFragmentsFromArguments():
    for arg in sys.argv[1:]:
        print()
        print(arg)
        img = cv2.imread(arg)
        if img is not None:
            print("boxTest: ")
            boxTest(img)
        else:
            raise Exception("File not an image: " + arg)


def main():
    if len(sys.argv) > 1:
        print("Arguments found", str(sys.argv))
        checkFragmentsFromArguments()
    else:
        print("No fragments folder found")
        raise Exception("No arguments given")


try:
    main()
except Exception as e:
    print("Exception thrown: ", e)
    input("Press any key to exit...")
