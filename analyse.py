import imutils
from imutils import contours
from imutils import perspective
from scipy.spatial import distance as dist
import os
import cv2
import numpy
import math
import sys
import json
import argparse


def getDirection(p1, p2):
    devPrint("p1: ", p1)
    devPrint("p2: ", p2)

    if p1[0] == p2[0]:
        return 90
    direction = math.atan((p1[1] - p2[1])/(p1[0] - p2[0])) * 180 / math.pi
    direction = direction % 180
    return direction


def midpoint(ptA, ptB):
    return ((ptA[0] + ptB[0]) * 0.5, (ptA[1] + ptB[1]) * 0.5)


def boxTest(arg):
    image = cv2.imread(arg)
    edged = cv2.Canny(image, 50, 100)
    # close gaps between object edges
    edged = cv2.dilate(edged, None, iterations=1)
    edged = cv2.erode(edged, None, iterations=1)
    # konturen finden
    cnts = cv2.findContours(edged.copy(), cv2.RETR_EXTERNAL,
                            cv2.CHAIN_APPROX_SIMPLE)
    cnts = imutils.grab_contours(cnts)
    (cnts, _) = contours.sort_contours(cnts)
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

        if args.development:
            # cv2.imshow("Image", orig)
            # cv2.waitKey(0)
            cv2.imwrite(arg + "box.png", orig)
        dA = dist.euclidean((tltrX, tltrY), (blbrX, blbrY))
        dB = dist.euclidean((tlblX, tlblY), (trbrX, trbrY))

        if dA > dB:
            angle = getDirection((tltrX, tltrY), (blbrX, blbrY))
        else:
            angle = getDirection((tlblX, tlblY), (trbrX, trbrY))

        # check if the box we got is too small
        if dA < (image.shape[0] * 0.9) and dB < (image.shape[1]*0.9):
            devPrint("shape 0: ", image.shape[0])
            devPrint("shape 1: ", image.shape[1])
            devPrint("dA: ", dA)
            devPrint("dB: ", dB)
            return json.dumps({
                "path": arg,
                "directionA": round(dA, 2),
                "directionB": round(dB, 2),
                "angle": round(angle, 2),
                "status": "error"
            })

        value = {
            "path": arg,
            "directionA": round(dA, 2),
            "directionB": round(dB, 2),
            "angle": round(angle, 2),
            "status": "success"
        }
        return json.dumps(value)


def checkFragmentsFromArgument(path):
    img = cv2.imread(path)
    if img is not None:
        try:
            return boxTest(path)
        except Exception as e:
            devPrint("Exception thrown: ", e)
    else:
        raise Exception("File not an image: " + path)


def main(path):
    return checkFragmentsFromArgument(path)


def devPrint(*arg, **kwargs):
    if args.development:
        print(*arg, **kwargs)


try:
    parser = argparse.ArgumentParser(description='Pass the path to the image')
    parser.add_argument('path', type=str)
    parser.add_argument("-d", "--development",
                        help="enable Dev mode", default=False, required=False)

    args = parser.parse_args()

    devPrint("Arguments: ", args)
    devPrint("System Arguments: ", sys.argv)

    if args.development:
        devPrint(main(args.path))
    else:
        main(args.path)
except Exception as e:
    devPrint("Exception thrown: ", e)
    input("Press any key to exit...")
