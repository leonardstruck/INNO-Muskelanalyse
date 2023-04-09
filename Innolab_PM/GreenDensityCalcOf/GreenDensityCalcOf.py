
# import necessary libraries
from skimage import feature, color, io, filters
import cv2
import matplotlib.pyplot as plt
import sys
import json
import numpy as np

#path as parameter

#filepath = sys.argv[0]



# io.imread supports png
image = io.imread('D:\INNOLAB\Input-20230331T200711Z-001\Inputtests_under 1GB\Screenshot 2023-04-02 170138.png')

# Convert image to HSV color space 
hsv_image = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)

lower_green = np.array([40, 40, 40])
upper_green = np.array([70, 255, 255])

# Erstelle eine Maske für die grüne Farbe
mask = cv2.inRange(hsv_image, lower_green, upper_green)

green_pixels = np.count_nonzero(mask)

total_pixels = image.shape[0] * image.shape[1]

green_density = (green_pixels / total_pixels) * 100
green_density_json = json.dumps({"green_density": green_density})

print(f"Green color density: {green_density:.2f}%")
