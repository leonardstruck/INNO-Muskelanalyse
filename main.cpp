#include <iostream>
#include <opencv2/opencv.hpp>
#include <vector>
#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include "segmentation.cpp"
#include "segmentor.cpp"
#include "segment.cpp"
#include <iostream>

using namespace cv;
Mat src, src_gray;
Mat dst, detected_edges;


std::vector<Segment*> segmente;

int averageGreen(Mat image, int rows, int cols)
{
    int green = 0;
    int counter = 0;
    for(int y = 0; y < rows; y++)
    {
        for(int x = 0; x < cols; x++)
        {
            Vec3b point = image.at<Vec3b>(y,x);
            if(point[1] > point[0] && point[1] > point[2])
            {
                green += point[1];
                counter++;
            }

        }
    }
    return (green/counter);
}

int maxGreen(Mat image, int rows, int cols)
{
    int green = 0;
    for(int y = 0; y < rows; y++)
    {
        for(int x = 0; x < cols; x++)
        {
            //if(image.at<Vec3b>(y,x)[1] > green)
            //{
                green = image.at<Vec3b>(y,x)[1];
            //}
        }
    }
    return green;
}

/*int map(int value, int oldMin, int oldMax, int newMin, int newMax)
{
    return ((value-oldMin) / (oldMax-oldMin)) * (newMax-newMin) + newMin;
}*/

/*int segment(int value, int avg, int max)
{
    if(value <= avg)
    {
        return std::map(value,0,avg,0,255/2);
    }
    else
    {
        return std::map(value,avg,max,255/2,255);
    
    }


}*/






int main(int, char**) {   

    auto started = std::chrono::high_resolution_clock::now();
    bool debugbool = false;

    Mat image;
    Mat channels[3];
    
    

    image = imread("D:/FH_offline/InnoLab/slices/test.png");

    

    if( !image.data)
    {
        printf("No image data \n");
        return -1;
    }





    

    int ** greenmap = new int*[image.rows];
    
    for(int i = 0; i < image.rows; i++)
    {
        greenmap[i] = new int[image.cols];

        
    }

    
    

    Mat compare;
    medianBlur( image, compare,  5 );    

    Mat result = compare.clone();


    int avg = averageGreen(image, result.rows,result.cols);
    int max = maxGreen(image, result.rows,result.cols);

    printf("AVG:%d\nMAX:%d\n",avg,max);
    int near = 0;
    for(int y = 0; y < compare.rows; y++)
    {
        for(int x = 0; x < compare.cols; x++)
        {
            
            Vec3b point = compare.at<Vec3b>(y,x);
           
            result.at<cv::Vec3b>(y,x)[0] = 0;
            result.at<cv::Vec3b>(y,x)[1] = 0;
            result.at<cv::Vec3b>(y,x)[2] = 0;
            greenmap[y][x] = 0;
           /*
            if(point[1] > point[0] && point[1] > point[2])
            {
                
                if(point[1] > avg*1.5 ){

                    if(near > 1)
                    {
                        result.at<cv::Vec3b>(y,x)[0] = 255;
                        result.at<cv::Vec3b>(y,x)[1] = 255;
                        result.at<cv::Vec3b>(y,x)[2] = 255;
                    } 

                
                    
                    greenmap[y][x] = 3;
                    near += 3;
                }
                else if(point[1] > avg*1.2 ){
                     
                    if(near > 2)
                    {
                        result.at<cv::Vec3b>(y,x)[0] = 255;
                        result.at<cv::Vec3b>(y,x)[1] = 255;
                        result.at<cv::Vec3b>(y,x)[2] = 255;
                    } 
                    
                    greenmap[y][x] = 2;
                    near += 2;
                }
                else if(point[1] > avg ){
                    
                    if(near > 3)
                    {
                        result.at<cv::Vec3b>(y,x)[0] = 255;
                        result.at<cv::Vec3b>(y,x)[1] = 255;
                        result.at<cv::Vec3b>(y,x)[2] = 255;
                    } 
                    
                    greenmap[y][x] = 1;
                    near += 1; 
                }
            }
            else if(point[1] > avg*0.5 )
            {
                    
                    // if(near > 3)
                    // {
                        result.at<cv::Vec3b>(y,x)[0] = 255;
                        result.at<cv::Vec3b>(y,x)[1] = 255;
                        result.at<cv::Vec3b>(y,x)[2] = 255;
                    //} 
                    
                    greenmap[y][x] = 0;
                    
            }
                //near = (near > 0) ? near-- : 0;
                */

            if(point[1] > point[0] && point[1] > point[2])
            {
                        result.at<cv::Vec3b>(y,x)[0] = 255;
                        result.at<cv::Vec3b>(y,x)[1] = 255;
                        result.at<cv::Vec3b>(y,x)[2] = 255;
                        greenmap[y][x] = 1;
                if(point[1] < avg*0.5)
                {
                        result.at<cv::Vec3b>(y,x)[0] = 0;
                        result.at<cv::Vec3b>(y,x)[1] = 0;
                        result.at<cv::Vec3b>(y,x)[2] = 0;
                        greenmap[y][x] = 0;
                }
            }
            
        }          

    }
    int border = 0;
    /*for(int y = 0; y < compare.rows; y++)
    {
        for(int x = 0; x < compare.cols; x++)
        {
            if(x > 0 && y > 0 && x < compare.cols && y < compare.rows)
            {
                if(greenmap[y][x] == 1)
                {
                    border = greenmap[y-1][x] + greenmap[y-1][x-1] + greenmap[y][x-1] + greenmap[y+1][x] + greenmap[y+1][x+1] + greenmap[y][x+1] + greenmap[y-1][x+1] + greenmap[y+1][x-1];
                    if(border != 0 && border < 8)
                    {
                         result.at<cv::Vec3b>(y,x)[0] = 0;
                         result.at<cv::Vec3b>(y,x)[1] = 255;
                         result.at<cv::Vec3b>(y,x)[2] = 0;

                         greenmap[y][x] = 4;
                    }
                }
            }
        }

    }*/

    
    Segmentor * harry = new Segmentor(&greenmap,0,result.rows,result.cols);
    
    printf("Segmente: %zd", harry->segmente.size());
    
       
    
    
   /*
    
    int currentseg = 0;
    for(std::vector<myoSegment>::iterator it = harry->segmente.begin(); it != harry->segmente.end(); ++it)
    {
        
        Mat segPic(it->height,it->width, CV_8UC3, Scalar(0,0,0));
        
        int counter = 0;
        for(int y = it->y; y < it->y + it->height; y++)
        {
            for(int x = it->x; x < it->x + it->width; x++)
            {
                if(it->map[y][x] == 1)
                {
                
                    segPic.at<cv::Vec3b>(y-it->y,x-it->x) = compare.at<cv::Vec3b>(y,x);
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
        String name = "segment " + std::to_string(++currentseg);
        name = name + ".jpg";
        namedWindow(name, WINDOW_AUTOSIZE);
        imshow(name, segPic);
        imwrite("D:/OneDrive - FH Technikum Wien/3.Semester/Inno/OPENCV-Test/segments/"+name,segPic);

        Point p1(it->x,it->y);
        Point p2(it->x+it->width,it->y+it->height);

        rectangle(src,p1,p2,Scalar(0,0,255),2,LINE_8);
        putText(compare, std::to_string(currentseg), Point(it->x + (it->width/2), it->y+it->height+20), FONT_HERSHEY_SIMPLEX, 0.5,Scalar(0,0,255),1);
        
    }*/
    /*
    bool found = false;
    for(int y = 0; y < result.rows; y++)
    {
        for(int x = 0; x < result.cols; x++)
        {
            if(greenmap[y][x] >= 1)
            {
                found = false;
                for(std::vector<Segment*>::iterator it = segmente.begin(); it != segmente.end(); ++it)
                {
                    if(it[0]->isNear(x,y, greenmap[y][x]))
                    {
                        it[0]->add(x,y);
                        found = true;
                        break;
                    }
                }
                if(!found)
                {
                    Segment * temp = new Segment(x,y);
                    segmente.push_back(temp);
                }
            }
        }
    }

    


    for(std::vector<Segment*>::iterator it = segmente.begin(); it != segmente.end(); ++it)
    {
        for(std::vector<Segment*>::iterator next = segmente.begin(); next != segmente.end(); ++next)
        {
            if(it[0]!=next[0] && it[0]->active && next[0]->active)
            {
                if(it[0]->intersectionCheck(next[0]) > 25)
                {
                    it[0]->active = false;
                    next[0]->addRect(it[0]);
                    break;
                }                
            }
        }
    }
    // for(std::vector<Segment*>::iterator it = segmente.begin(); it != segmente.end(); ++it)
    // {
    //     if(it[0]->size() > 60 && it[0]->active)
    //     {

    //        Mat seg = result(Rect(it[0]->minx,it[0]->miny,it[0]->maxx-it[0]->minx,it[0]->maxy-it[0]->miny));
    //        imwrite("D:/FH_offline/InnoLab/segmente/seg" + to_string(it[0]->minx) + "-" + to_string(it[0]->minx) + ".jpg", seg);
    //     }
    // }

    for(std::vector<Segment*>::iterator it = segmente.begin(); it != segmente.end(); ++it)
    {
        if(it[0]->size() > 60 && it[0]->active)
        {
            Point p1(it[0]->minx,it[0]->miny);
            Point p2(it[0]->maxx,it[0]->maxy);
            rectangle(compare,p1,p2,Scalar(0,0,255),2,LINE_8);
            rectangle(result,p1,p2,Scalar(0,0,255),2,LINE_8);
        }
    }*/

    for(std::vector<myoSegment>::iterator it = harry->segmente.begin(); it != harry->segmente.end(); ++it)
    {

        if(it[0].height * it[0].width > 300)
        {
            //printf("Size: %dx%d\n", it[0].height,it[0].width);
            
            Mat seg(it[0].height,it[0].width,  CV_8UC3);

            //printf("Matsize: %dx%d \n",seg.rows,seg.cols);
            for(int y = 0; y < it[0].height; y++)
            {
                for(int x = 0; x < it[0].width; x++)
                {
                    if(x >= 0 && x < seg.cols && y >= 0 && y < seg.rows)
                    {
                        try
                        {
                            seg.at<cv::Vec3b>(y,x)[0] = 255 * it[0].map[y][x];
                            seg.at<cv::Vec3b>(y,x)[1] = 255 * it[0].map[y][x];
                            seg.at<cv::Vec3b>(y,x)[2] = 255 * it[0].map[y][x];
                        }
                        catch(const std::exception& e)
                        {
                            //printf("%d:%d \n",y,x);
                            std::cerr << e.what() << '\n';
                        }   
                    }
                    else
                    {
                        //printf("Outside: %d:%d Max-Area: %d:%d\n", y,x,seg.rows,seg.cols);
                    }
                    
                }
            }
           
            //namedWindow("Display Compare2"+ to_string(it[0].minX) + "-" + to_string(it[0].minY), WINDOW_NORMAL);
            //imshow("Display Compare2"+ to_string(it[0].minX) + "-" + to_string(it[0].minY), seg);
            imwrite("D:/FH_offline/InnoLab/segmente/seg" + to_string(it[0].minX) + "-" + to_string(it[0].minY) + ".jpg", seg);
        }
    }
    

    

    


        
    
    //imwrite("D:/FH_offline/InnoLab/slices/export3.jpg", result);
    // namedWindow("Display Compare1", WINDOW_NORMAL);
    // imshow("Display Compare1", compare);

    // namedWindow("Display Compare2", WINDOW_NORMAL);
    // imshow("Display Compare2", result);
    auto done = std::chrono::high_resolution_clock::now();
    std::cout << "\nMilliseconds: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "\n";
    waitKey(0);
    return 0;
}




// int main( int argc, char** argv )
// {
  
//   src = imread( samples::findFile("D:/FH_offline/InnoLab/slices/test.png"), IMREAD_COLOR ); // Load an image
//   if( src.empty() )
//   {
//     std::cout << "Could not open or find the image!\n" << std::endl;
//     std::cout << "Usage: " << argv[0] << " <Input image>" << std::endl;
//     return -1;
//   }
  
// }