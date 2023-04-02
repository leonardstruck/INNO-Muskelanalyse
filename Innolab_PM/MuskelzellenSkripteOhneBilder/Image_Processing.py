#Program converts the png image into an jpg and saves different variants (border and morphology) in the corresponding folders of the jpg image

import cv2
from matplotlib import pyplot as plt
import PIL
import numpy as np

PIL.Image.MAX_IMAGE_PIXELS = 933120001 

#Read the image
img = cv2.imread('./Colored png/LOLU.png')

#Convert the image from BGR to RGB
img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

#Convert the image to grayscale
imgGrayscale = cv2.cvtColor(img, cv2.COLOR_RGB2GRAY)

imgBlurred = cv2.GaussianBlur(imgGrayscale, (3,3), 0)

_, imgThresholded = cv2.threshold(imgBlurred, 0, 255, cv2.THRESH_OTSU | cv2.THRESH_BINARY_INV)

#Find the contours
contours, _ = cv2.findContours(imgThresholded, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)

#Draw the contours
cv2.drawContours(img, contours, -1, (0, 255, 0), 3)

cv2.imwrite('./Border/LOLU_contours.jpg', img)

#Show the contours
plt.imshow(img)
plt.show()