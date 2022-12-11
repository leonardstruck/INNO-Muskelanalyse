#include <iostream>
#include <opencv2/opencv.hpp>
#include <vector>
#include "segmentation.cpp"

using namespace cv;




int averageGreen(Mat image, int rows, int cols)
{
    int green = 0;
    for(int y = 0; y < rows; y++)
    {
        for(int x = 0; x < cols; x++)
        {
            green += image.at<Vec3b>(y,x)[1];
        }
    }
    return (green/(rows*cols));
}

int maxGreen(Mat image, int rows, int cols)
{
    int green = 0;
    for(int y = 0; y < rows; y++)
    {
        for(int x = 0; x < cols; x++)
        {
            if(image.at<Vec3b>(y,x)[1] > green)
            {
                green = image.at<Vec3b>(y,x)[1];
            }
        }
    }
    return green;
}

int map(int value, int oldMin, int oldMax, int newMin, int newMax)
{
    return ((value-oldMin) / (oldMax-oldMin)) * (newMax-newMin) + newMin;
}

int segment(int value, int avg, int max)
{
    if(value <= avg)
    {
        return map(value,0,avg,0,255/2);
    }
    else
    {
        return map(value,avg,max,255/2,255);
    }


}






int main(int, char**) {
    std::vector<myoSegment> segments; 
    
     
    int ratio = 1000;
    int ** greenmap = new int*[ratio];
    
    for(int i = 0; i < ratio; i++)
    {
        greenmap[i] = new int[ratio];
    }



    bool debugbool = false;

    Mat image;
    Mat channels[3];
    
    Mat result(ratio,ratio, CV_8UC3, Scalar(10,100,150));

    image = imread("D:/FH_offline/InnoLab/slices/test.png");

    

    if( !image.data)
    {
        printf("No image data \n");
        return -1;
    }
    Mat compare (image,Rect(0,0,ratio,ratio));
    split(image,channels);

    int avg = averageGreen(image, ratio, ratio);
    int max = maxGreen(image, ratio, ratio);

    printf("AVG:%d\nMAX:%d\n",avg,max);
    for(int y = 0; y < ratio; y++)
    {
        for(int x = 0; x < ratio; x++)
        {
            Vec3b point = image.at<Vec3b>(y,x);
           
                        
           
            if(point[1] > point[0] && point[1] > point[2])
            {
                result.at<cv::Vec3b>(y,x)[0] = 0;
                result.at<cv::Vec3b>(y,x)[1] = segment(point[1],avg,max);
                result.at<cv::Vec3b>(y,x)[2] = 0;
                if(point[1] > avg ){greenmap[y][x] = 1;}
                else{greenmap[y][x] = 0;}
                
            }
            else
            {
                result.at<cv::Vec3b>(y,x)[0] = 0;
                result.at<cv::Vec3b>(y,x)[1] = 0;
                result.at<cv::Vec3b>(y,x)[2] = 0;
                greenmap[y][x] = 0;
          
            }
           

        }
    }
    /*printf("   ");
    for(int x = 0; x < ratio; x++)
    {
        printf("%2d|",x);
    }
    printf("\n");
    for(int y = 0; y < ratio; y++)
    {
        printf("%3d:",y);
        for(int x = 0; x < ratio; x++)
        {
           if(greenmap[y][x] == 1)
           {
            printf(" 1 ");
           }
           else
           {
            printf(" - ");
           }
        }
        printf("\n");
    }

    printf("\n");
    printf("\n");*/
    for(int y = 0; y < ratio; y++)
    {
        for(int x = 0; x < ratio; x++)
        {
           if(greenmap[y][x] == 1 && segments.size() < 5)
           {
            // printf("segmentation at %d:%d\n",y,x);
            // printf("value:%d\n",greenmap[y][x]);
            //printf("Y:%d\n",y);
            printf("-------------\n");
            printf("Y:%d - X:%d\n",y,x);
            printf("Segments:%d\n",(int)segments.size());
            segments.push_back(startSegmentation(x,y,&greenmap,0,0,ratio,ratio));
            y = 0;
            x = segments.back().width;
            if(segments.back().height < 10 || segments.back().width < 10) {segments.pop_back();}
            
   
            //greenmap[y][x] = 2;
           }
        }
     
    }
    
    
       
    for(std::vector<myoSegment>::iterator it = segments.begin(); it != segments.end(); ++it)
    { 
        for(int x = 0; x < it->width; x++)
        {
            printf("%2d|",x);
        }
        printf("\n");
        for(int y = it->y; y < it->y + it->height; y++)
        {
            printf("%3d:",y-it->y);
            for(int x = it->x; x < it->x + it->width; x++)
            {
            if(it->map[y][x] == 1)
            {
                printf(" 1 ");
            }
            else
            {
                printf(" - ");
            }
            }
            printf("\n");
        }

        printf("###################################\n");
    }

    int seg = (int)segments.size();
   
    
    int currentseg = 0;
    for(std::vector<myoSegment>::iterator it = segments.begin(); it != segments.end(); ++it)
    {
        
        Mat segPic(it->height,it->width, CV_8UC3, Scalar(10,100,150));
        int ** newMap = it->map;
        int counter = 0;
        for(int y = it->y; y < it->y + it->height; y++)
        {
            for(int x = it->x; x < it->x + it->width; x++)
            {
                if(newMap[y][x] == 1)
                {
                
                    segPic.at<cv::Vec3b>(y-it->y,x-it->x) = image.at<cv::Vec3b>(y,x);
                }
                else
                {
                    counter++;
                    segPic.at<cv::Vec3b>(y-it->y,x-it->x)[0] = 0;
                    segPic.at<cv::Vec3b>(y-it->y,x-it->x)[1] = 0;
                    segPic.at<cv::Vec3b>(y-it->y,x-it->x)[2] = 0;

                }
            }
        } 
        String name = "segment " + currentseg++;
        name = name + ".jpg";
        namedWindow(name, WINDOW_AUTOSIZE);
        imshow(name, segPic);
        imwrite("segments/"+name,segPic);
        
    }
    

    
    
    namedWindow("Display Compare", WINDOW_AUTOSIZE);
    imshow("Display Compare", compare);

    waitKey(0);
    return 0;
}
