#Read the image from the location.
#As a colored image has RGB layers in it and is more complex, convert it to its Grayscale form first.
#Set up a Threshold mark, pixels above the given mark will turn white, and below the mark will turn black.
#Save image in folder "Morphology" 

import cv2

# read the image file
img = cv2.imread("./jpg/LORO.jpg", 2)

ret, bw_img = cv2.threshold(img, 127, 255, cv2.THRESH_BINARY)

# converting to its binary form
bw = cv2.threshold(img, 127, 255, cv2.THRESH_BINARY)

cv2.imshow("Binary", bw_img)
cv2.waitKey(0)
cv2.destroyAllWindows()