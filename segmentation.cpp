#pragma once
#include <vector>

struct myoSegment{
    int x;
    int y;
    int width;
    int height;
    
    int ** map;
    int minX;
    int maxX;
    int minY;
    int maxY;
} typedef myoSegment;


/*

void  contiSegmentation(int x, int y, int *** greenmap, myoSegment * newSegment)
{
   
    if(x >= newSegment->minX && y >= newSegment->minY && x <= newSegment->maxX && y <= newSegment->maxY) 
    {
    
        if(greenmap[0][y][x] == 1)
        {
            if(x < newSegment->x){newSegment->x = x;}
            if(y < newSegment->y){newSegment->y = y;}
            if(y - newSegment->y > newSegment->height){newSegment->height = y - newSegment->y;}
            if(x - newSegment->x > newSegment->width){newSegment->width = x - newSegment->x;}

            newSegment->map[y][x] = 1;
            greenmap[0][y][x] = 0;
       
            contiSegmentation(x-1,y-1,greenmap,newSegment);
            contiSegmentation(x,y-1,greenmap,newSegment);
            contiSegmentation(x,y+1,greenmap,newSegment);

            contiSegmentation(x-1,y,greenmap,newSegment);
            contiSegmentation(x+1,y,greenmap,newSegment);

            contiSegmentation(x-1,y+1,greenmap,newSegment);
            contiSegmentation(x,y+1,greenmap,newSegment);
            contiSegmentation(x+1,y+1,greenmap,newSegment);
            
        }
        
    }
    

}
myoSegment  startSegmentation(int x, int y, int *** greenmap,  int minX, int minY, int maxX, int maxY)
{
    

    myoSegment newSegment;
    newSegment.x = x;
    newSegment.y = y;
    newSegment.width = 0;
    newSegment.height = 0;

    newSegment.minX = minX;
    newSegment.minY = minY;
    newSegment.maxX = maxX;
    newSegment.maxY = maxY;
    
    newSegment.map = new int*[maxY-minY];
  
    for(int i = 0; i < maxY-minY; i++)
    {
        newSegment.map[i] = new int[maxX-minX];
    }
    
    contiSegmentation(x,y,greenmap,&newSegment);
    
    return newSegment;
}*/