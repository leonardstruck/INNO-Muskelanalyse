# import necessary libraries
from skimage import feature, color, io
import cv2
import matplotlib.pyplot as plt
import sys
import json
import numpy as np

def detectBlobsErosion(image):
    nr_of_found_blobs = 0

    image = image.astype(np.uint8)
    gray_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (4, 4))

    # Perform erosion
    eroded = cv2.erode(gray_image, kernel, iterations=1)
    eroded = eroded.astype(np.uint8)

    # Find the contours of the blobs in the mask
    contours, _ = cv2.findContours(eroded, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    # Create a list to store the coordinates of the labels
    coordinatesAndOtherInfo = []

    # Draw a red circle around the center of each blob
    for contour in contours:
        M = cv2.moments(contour)
        if M["m00"] != 0:
            cX    = int(M["m10"] / M["m00"])
            cY    = int(M["m01"] / M["m00"])
            cSize = cv2.contourArea(contour)
            
            perimeter = cv2.arcLength(contour, True)
            radius = perimeter / (2 * np.pi)

            surrounding_circle_area = np.pi * radius**2

            cCompactness = cSize / surrounding_circle_area

            cxMoment = cyMoment = 0

            if len(contours) > 0 and cv2.contourArea(contours[0]) > 0:
                contourMoments = contours[0]
                moments = cv2.moments(contourMoments)
                cxMoment = int(moments["m10"] / moments["m00"])
                cyMoment = int(moments["m01"] / moments["m00"])
                
            (x,y),radius = cv2.minEnclosingCircle(contour)
            center = (int(x),int(y))

            blobCoord = (int(cX), int(cY))
            momentCoord = (int(cxMoment), int(cyMoment))

            differenceCoord = (momentCoord[0] - blobCoord[0], momentCoord[1] - blobCoord[1])

            if (cCompactness < 0.5 or cSize > 10000) and (differenceCoord[0] < 10 and differenceCoord[0] > -10 and differenceCoord[1] < 10 and differenceCoord[1] > -10):
                cv2.circle(image, (cX, cY), 10, (0, 0, 255), 2)
                cv2.circle(image, (cX, cY), 10, (0, 0, 255), 2)
                coordinatesAndOtherInfo.append((blobCoord, cSize, cCompactness, momentCoord, center))
                nr_of_found_blobs += 1
            
    # Print the number and coordinates of the labels
    print("Number of blobs detected:", nr_of_found_blobs)
    print("Information of the labels:", coordinatesAndOtherInfo)

    # Show the image
    cv2.imshow('Blobs', image)
    cv2.waitKey(0)

    ev_JSON_coordinate_string = json.dumps(coordinatesAndOtherInfo)

    nr_of_found_blobs_string = "%d" % nr_of_found_blobs
    
    #Return JSON coordinatesAndOtherInfo (Coordinates, size, compactness, coordinates of main emphasis) and number of found blobs, 
    sys.stdout.write(ev_JSON_coordinate_string + '\n' + nr_of_found_blobs_string + ' ')

def detectBlobs(mask):
    # Use morphological operations to remove noise and fill small gaps
    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (5,5))
    mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)

    # Find the contours of the blobs in the mask
    contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    # Create a list to store the coordinates of the labels
    coordinates = []

    # Draw a red circle around the center of each blob
    for contour in contours:
        M = cv2.moments(contour)
        if M["m00"] != 0:
            cX = int(M["m10"] / M["m00"])
            cY = int(M["m01"] / M["m00"])
            cv2.circle(image, (cX, cY), 10, (0, 0, 255), 2)
            coordinates.append((cX, cY))

    nr_of_found_blobs = len(contours)
    # Print the number and coordinates of the labels
    print("Number of blobs detected:", nr_of_found_blobs)
    print("Coordinates of the labels:", coordinates)

    # Show the image
    cv2.imshow('Blobs', image)
    cv2.waitKey(0)

    ev_JSON_coordinate_string = json.dumps(coordinates)

    nr_of_found_blobs_string = "%d" % nr_of_found_blobs
    
    #Return JSON coordinates and number of found blobs
    sys.stdout.write(ev_JSON_coordinate_string + '\n' + nr_of_found_blobs_string + ' ')

#path as parameter

filepath = sys.argv[0]

if len(sys.argv) < 2:
    print("Usage: python script.py path/to/image")
    sys.exit()

# convert the image to HSV color space
image = io.imread(filepath)
imageBinary = cv2.imread(filepath, -1)
hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)

# Flatten the image to a 1D array
img_flat = image.reshape(-1, image.shape[-1])

# Get the unique colors of the image
unique_colors, counts = np.unique(img_flat, return_counts=True, axis=0)

num_of_colors = len(unique_colors)

# Check if the image is binary or colored
if num_of_colors < 20:
    detectBlobsErosion(image)
    
else:
    lower_border = (20, 30, 30)
    upper_border = (100, 255, 255)
    # Threshold the image to get only green colors
    mask = cv2.inRange(hsv, lower_border, upper_border)
    detectBlobs(mask)






