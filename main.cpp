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
#include <fstream>
#include <string>


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
            if(image.at<Vec3b>(y,x)[1] > green)
            {
                green = image.at<Vec3b>(y,x)[1];
            }
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

inline bool fileExists(const std::string& name) {
    ifstream f(name.c_str());
    return f.good();
}






int main(int argc, char *argv[]) {   


    const string picture = "D:/FH_offline/InnoLab/slices/test.png";
    string folder = "D:/FH_offline/InnoLab/newSegmente";
    string jsonFolder = "D:/FH_offline/InnoLab/";
    string jsonFile = "segmente.json";

    //check parameters
    if(argc >= 3)
    {
        const string picture = argv[1];
        string folder = argv[2];
        string jsonFolder = argv[2];
        string jsonFile = "segmente.json";

        if(argc > 3)
        {
            jsonFolder = argv[3];
        }

        folder = (folder.back() == '/') ? folder : folder+"/";
        
        

        if(!fileExists(picture))
        {
            printf("file: '%s' could not be opened\n",picture.c_str());
            return -1;
        }
    }

    

    auto started = std::chrono::high_resolution_clock::now();
    bool debugbool = false;

    Mat image;
    Mat channels[3];
    
    

    image = imread(picture);

    

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
    //int max = maxGreen(image, result.rows,result.cols);

    //printf("AVG:%d\nMAX:%d\n",avg,max);
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


    //              border 
    /*int border = 0;
    for(int y = 0; y < compare.rows; y++)
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
    
    //printf("Segmente: %zd", harry->segmente.size());
    
       
    
    // blob detection
   /*int currentseg = 0;
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
    
    

    
    jsonFile = (jsonFolder.back() != '/') ? jsonFolder : jsonFolder+"/"+jsonFile;
    
    

    ofstream outputJson(jsonFile);

    if(!fileExists(jsonFile))
    {
        printf("JSON: \"%s\" could not be created",jsonFile.c_str());
        return -1;
    }
    
    outputJson << "[\n";
    printf("[\n");
    for(std::vector<myoSegment>::iterator it = harry->segmente.begin(); it != harry->segmente.end(); ++it)
    {

        if(it[0].height * it[0].width > 300)
        {
            
            
            Mat seg(it[0].height,it[0].width,  CV_8UC3);

           
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
           
            if(it != harry->segmente.begin() && it != harry->segmente.end())
            {
                outputJson << ",\n";
                printf(",\n");
            }
            string path = folder + "seg" + to_string(it[0].minX) + "-" + to_string(it[0].minY) + ".jpg";

            outputJson << "{\"path\":\""+path+"\",\"y\":"+to_string(it[0].minY)+",\"x\":"+to_string(it[0].minX)+",\"height\":"+to_string(it[0].height)+",\"width\":"+to_string(it[0].width)+"}";


            printf("{\"path\":\"%s\",\"y\":%d,\"x\":%d,\"height\":%d,\"width\":%d}",path.c_str(),it[0].minY,it[0].minX,it[0].height,it[0].width);
            imwrite(path, seg);
        }
    }
    printf("\n]");
    outputJson << "\n]";

    outputJson.close();

    for(std::vector<myoSegment>::iterator it = harry->segmente.begin(); it != harry->segmente.end(); ++it)
    {
        if(it[0].height * it[0].width > 300)
        {
            Point p1(it[0].minX,it[0].minY);
            Point p2(it[0].minX+it[0].width,it[0].minY+it[0].height);

            rectangle(image,p1,p2,Scalar(0,0,255),2,LINE_8);
        }
    }

    namedWindow("Display Compare", WINDOW_NORMAL);
    imshow("Display Compare", image);


    /*Segment:
    string path,
    int x, 
    int y,
    int length,
    int width,*/

    


        
    
    //imwrite("D:/FH_offline/InnoLab/slices/export3.jpg", result);
    namedWindow("Display Compare", WINDOW_NORMAL);
    imshow("Display Compare", image);

    // namedWindow("Display Compare2", WINDOW_NORMAL);
    // imshow("Display Compare2", result);
    auto done = std::chrono::high_resolution_clock::now();
    //std::cout << "\nMilliseconds: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "\n";
    waitKey(0);
    return 0;

}