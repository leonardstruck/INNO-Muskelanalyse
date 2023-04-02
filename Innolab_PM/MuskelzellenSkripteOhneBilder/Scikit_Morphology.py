import cv2
import matplotlib.pyplot as plt
from skimage import io, filters, color, morphology, measure

# read in the image and convert it to grayscale
image = io.imread('./MuscleCellsOverlap/jpg/LORO.jpg')
image_gray = io.imread('./MuscleCellsOverlap/GrayscaleJPG/LOROGRAYSCALE.jpg')

# apply a threshold to the image to segment the cells
threshold = filters.threshold_otsu(image_gray)
segmented_cells = image_gray > threshold

# use morphological operations to clean up the segmentation
segmented_cells = morphology.remove_small_objects(segmented_cells, min_size=64)
segmented_cells = morphology.remove_small_holes(segmented_cells, area_threshold=64)

# use connected component analysis to identify individual cells
from skimage import measure
labels = measure.label(segmented_cells, background=0)

plt.imshow(labels, cmap='gray')
plt.axis('off')

io.imsave("./MuscleCellsOverlap/Morphology/labeled_image_LORO.png", labels)

plt.show()

# find the size and centroid of each cell
props = measure.regionprops(labels)
for prop in props:
    print(f"Cell size: {prop.area}")
    print(f"Cell centroid: {prop.centroid}")
