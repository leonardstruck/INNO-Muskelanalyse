import cv2
import numpy as np
from skimage.measure import label, regionprops

def preprocess_image(image_path):
    image = cv2.imread(image_path)

    if image is None or image.size == 0:
        raise ValueError(f"Image could not be loaded. Check the file path: {image_path}")

    hsv_image = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    
    lower_green = np.array([40, 40, 40])
    upper_green = np.array([70, 255, 255])
    
    green_mask = cv2.inRange(hsv_image, lower_green, upper_green)
    return green_mask

def detect_overlapping_cells(binary_image, min_area=500, overlap_threshold=0.3):
    label_image = label(binary_image)
    overlapping_cells = []
    
    for region1 in regionprops(label_image):
        if region1.area < min_area:
            continue
        for region2 in regionprops(label_image):
            if region2.area < min_area or region1.label == region2.label:
                continue
            intersection_area = np.sum((label_image == region1.label) & (label_image == region2.label))
            union_area = region1.area + region2.area - intersection_area
            overlap_ratio = intersection_area / union_area
            
            if overlap_ratio > overlap_threshold:
                overlapping_cells.append((region1, region2))
                
    return overlapping_cells

def main():
    image_path = 'D:/INNOLAB/Input-20230331T200711Z-001/Inputtests_under 1GB/Screenshot 2023-04-02 170138.png'
    green_mask = preprocess_image(image_path)
    overlapping_cells = detect_overlapping_cells(green_mask)
    
    print(f'Number of overlapping cell pairs: {len(overlapping_cells)}')
    for i, (region1, region2) in enumerate(overlapping_cells, start=1):
        print(f'Overlapping pair {i}: Cell {region1.label} and Cell {region2.label}')

if __name__ == '__main__':
    main()
