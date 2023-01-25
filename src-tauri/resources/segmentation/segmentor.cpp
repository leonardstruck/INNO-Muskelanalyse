#include <vector>
#include "segmentation.cpp"
#include <exception>
#include <tuple>
#include <array>


using namespace std;

class Segmentor{
    public:
        Segmentor(int *** greenmap, int iterations, int height, int width);
        ~Segmentor();
        
        std::vector<myoSegment> segmente;

    private:
        int ** map;
        int mapLength;
        int mapHeight;
        void segmentation(int iterations);
        bool getSegment();
        void contiSegmentation(int x, int y, int search, int depth);
        void edgewalker(int startX, int startY);
        myoSegment createSegment(int *** map, int y, int x);
};


Segmentor::Segmentor(int *** greenmap, int iterations, int height, int width)
{
    //printf("\nSegmentor created!\n");
    map = greenmap[0];
    mapHeight = height;
    mapLength = width;


    bool iter = (iterations <= 0) ? true : false;
    int counter = 0;

    while((iter || counter < iterations) && getSegment())
    {
        counter++;
    }
    
}

bool Segmentor::getSegment()
{
   
   
    myoSegment newSegment;
    
   newSegment.map = new int*[mapHeight];
   for(int y = 0; y < mapHeight; y++)
   {
        newSegment.map[y] = new int[mapLength];
   }
   for(int y = 0; y < mapHeight; y++)
   {
        newSegment.height = 0;
        newSegment.width = 0;
        for(int x = 0; x < mapLength; x++)
        {
            if(map[y][x] == 1)
            {
                newSegment = createSegment(&map, y,x);
                segmente.push_back(newSegment);
                
            }
                
        }
        
    }
    
    return false;
    
    
    

}


myoSegment Segmentor::createSegment(int *** map, int y, int x)
{
    myoSegment * seg = new myoSegment();

    vector<tuple<int,int>> openPixels;
    vector<tuple<int,int>> segmentPixels;

    tuple<int,int> currentPixel = make_tuple(y, x);
    
   
   
    openPixels.push_back(currentPixel);
    segmentPixels.push_back(currentPixel);
    map[0][y][x] = 0;

    
    while(openPixels.size() > 0)
    {
        
        if(get<0>(currentPixel) + 1 < mapHeight)
        {

            //          [ ][ ][ ]
            //          [ ][ ][ ]
            //          [ ][x][ ]
            if(map[0][get<0>(currentPixel) + 1][get<1>(currentPixel)] >= 1)
            {
                currentPixel = make_tuple(get<0>(currentPixel) + 1, get<1>(currentPixel));
                openPixels.push_back(currentPixel);
                segmentPixels.push_back(currentPixel);
                map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                continue;
            }


            
            if(get<1>(currentPixel) - 1 > 0)
            {
                //          [ ][ ][ ]
                //          [ ][ ][ ]
                //          [x][ ][ ]
                if(map[0][get<0>(currentPixel)+1][get<1>(currentPixel)-1] >= 1)
                {
                    currentPixel = make_tuple(get<0>(currentPixel) + 1, get<1>(currentPixel) - 1);
                    openPixels.push_back(currentPixel);
                    segmentPixels.push_back(currentPixel);
                    map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                    continue;
                }
            }
            if(get<1>(currentPixel) + 1 < mapLength)
            {
                //          [ ][ ][ ]
                //          [ ][ ][ ]
                //          [ ][ ][x]     
                if(map[0][get<0>(currentPixel) + 1][get<1>(currentPixel) + 1] >= 1)
                {
                    currentPixel = make_tuple(get<0>(currentPixel) + 1, get<1>(currentPixel) + 1);
                    openPixels.push_back(currentPixel);
                    segmentPixels.push_back(currentPixel);
                    map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                    continue;
                }
            }
        }

        if(get<1>(currentPixel) - 1 > 0)
        {
            //          [ ][ ][ ]
            //          [x][ ][ ]
            //          [ ][ ][ ]
            if(map[0][get<0>(currentPixel)][get<1>(currentPixel)-1] >= 1)
            {
                currentPixel = make_tuple(get<0>(currentPixel), get<1>(currentPixel) - 1);
                openPixels.push_back(currentPixel);
                segmentPixels.push_back(currentPixel);
                map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                continue;
            }
        }

        if(get<1>(currentPixel) + 1 < mapLength)
        {
            //          [ ][ ][ ]
            //          [ ][ ][x]
            //          [ ][ ][ ]
            if(map[0][get<0>(currentPixel)][get<1>(currentPixel) + 1] >= 1)
            {
                currentPixel = make_tuple(get<0>(currentPixel), get<1>(currentPixel) + 1);
                openPixels.push_back(currentPixel);
                segmentPixels.push_back(currentPixel);
                map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                continue;
            }
        }
        if(get<0>(currentPixel) - 1 > 0)
        {
            if(get<1>(currentPixel) + 1 < mapLength)
            {
                //          [ ][ ][x]
                //          [ ][ ][ ]
                //          [ ][ ][ ]
                if(map[0][get<0>(currentPixel)-1][get<1>(currentPixel)+1] >= 1)
                {
                    currentPixel = make_tuple(get<0>(currentPixel) - 1, get<1>(currentPixel) + 1);
                    openPixels.push_back(currentPixel);
                    segmentPixels.push_back(currentPixel);
                    map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                    continue;
                }
            }

            //          [ ][x][ ]
            //          [ ][ ][ ]
            //          [ ][ ][ ]
            if(map[0][get<0>(currentPixel)-1][get<1>(currentPixel)] >= 1)
            {
                currentPixel = make_tuple(get<0>(currentPixel) - 1, get<1>(currentPixel));
                openPixels.push_back(currentPixel);
                segmentPixels.push_back(currentPixel);
                map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                continue;
            }
            if(get<1>(currentPixel) - 1 > 0)
            {
                //          [x][ ][ ]
                //          [ ][ ][ ]
                //          [ ][ ][ ]
                if(map[0][get<0>(currentPixel)-1][get<1>(currentPixel)-1] >= 1)
                {
                    currentPixel = make_tuple(get<0>(currentPixel) - 1, get<1>(currentPixel) - 1);
                    openPixels.push_back(currentPixel);
                    segmentPixels.push_back(currentPixel);
                    map[0][get<0>(currentPixel)][get<1>(currentPixel)] = 0;
                    continue;
                }
            }
        }
        

        

        currentPixel = openPixels.back();
        openPixels.pop_back();
    }
    seg->minY = get<0>(currentPixel);
    seg->minX = get<1>(currentPixel);

    seg->maxY = get<0>(currentPixel);
    seg->maxX = get<1>(currentPixel);

    for(vector<tuple<int, int>>::iterator it = segmentPixels.begin(); it != segmentPixels.end(); ++it )
    {
        if(get<0>(it[0]) < seg->minY)
        {
            seg->minY = get<0>(it[0]);
        }
        if(get<0>(it[0]) > seg->maxY)
        {
            seg->maxY = get<0>(it[0]);
        }

        if(get<1>(it[0]) < seg->minX)
        {
            seg->minX = get<1>(it[0]);
        }
        if(get<1>(it[0]) > seg->maxX)
        {
            seg->maxX = get<1>(it[0]);
        }
    }
    //printf("Min: %d:%d, Max %d:%d \n",seg->minX,seg->minY,seg->maxX,seg->maxY);
    seg->width = seg->maxX - seg->minX;
    seg->height = seg->maxY - seg->minY;
    seg->map = new int*[seg->height];
    for(int i = 0; i < seg->height; i++)
    {
        seg->map[i] = new int[seg->width];
    }
    for(int y = 0; y < seg->height; y++)
    {
        for(int x = 0; x < seg->width; x++)
        {
            seg->map[y][x] = 0;
        }
    }

    
    for(vector<tuple<int, int>>::iterator it = segmentPixels.begin(); it != segmentPixels.end(); ++it )
    {
       
        if(get<0>(it[0]) - seg->minY >= 0 && get<0>(it[0]) - seg->minY < seg->height && get<1>(it[0]) - seg->minX >= 0 && get<1>(it[0]) - seg->minX < seg->width)
        {
            seg->map[get<0>(it[0]) - seg->minY][get<1>(it[0]) - seg->minX] = 1;
        }
    }

    return *seg;
}




void Segmentor::contiSegmentation(int x, int y, int search, int depth)
{
    
        if(x >= 0 && y >= 0 && x <= mapLength && y <= mapHeight && depth < 2) 
        {
            
            if(map[y][x] >= 1)
            {
                if(x < segmente.back().x){segmente.back().x = x;}
                if(y < segmente.back().y){segmente.back().y = y;}
                if(y - segmente.back().y > segmente.back().height){segmente.back().height = y - segmente.back().y;}
                if(x - segmente.back().x > segmente.back().width){segmente.back().width = x - segmente.back().x;}

                map[y][x] = 0;
                segmente.back().map[y][x] = 1;
                if(map[y-search][x]>=1){contiSegmentation(x,y-search,search,depth);}
                if(map[y][x-search]>=1){contiSegmentation(x-search,y,search,depth);}
                if(map[y][x+search]>=1){contiSegmentation(x+search,y,search,depth);}
                if(map[y+search][x]>=1){contiSegmentation(x,y+search,search,depth);}
            }            
        }
}





