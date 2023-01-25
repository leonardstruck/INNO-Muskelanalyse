#include <math.h>


using namespace std;
class Segment{
    public:
        Segment(int x, int y);
        

        bool isNear(int x, int y, int whiteness);
        void add(int x, int y);
        int size();
        int intersectionCheck(Segment * overlap);
        void addRect(Segment * overlap);

        bool active;
        int minx;
        int miny;
        int maxx;
        int maxy;
        
    private:

        int threshold = 80;
        float distance(int x_1, int y_1, int x_2, int y_2);
};


Segment::Segment(int x, int y)
{
    minx = x;
    miny = y;
    maxx = x;
    maxy = y;

    active = true;
}

bool Segment::isNear(int x, int y, int whiteness)
{
    int centerx = (minx + maxx) / 2;
    int centery = (miny + maxy) / 2;

    if(distance(centerx,centery,x,y) < threshold + ((threshold/3)*whiteness))
    {
        return true;
    }
    if(distance(minx,miny,x,y) < threshold / (threshold/whiteness))
    {
        return true;
    }
    if(distance(maxx,miny,x,y) < threshold / (threshold/whiteness))
    {
        return true;
    }
    if(distance(minx,maxy,x,y) < threshold / (threshold/whiteness))
    {
        return true;
    }
    if(distance(maxx,maxy,x,y) < threshold / (threshold/whiteness))
    {
        return true;
    }
    return false;
    
}

float Segment::distance(int x_1, int y_1, int x_2, int y_2)
{
    return (float) sqrt((x_2-x_1) * (x_2-x_1) + (y_2 - y_1) * (y_2 - y_1));
}

void Segment::add(int x, int y)
{   
    minx = min(x,minx);
    maxx = max(x,maxx);

    miny = min(y, miny);
    maxy = max(y, maxy);
    
}

int Segment::size()
{
    return ((maxx-minx) * (maxy-miny));
}

int Segment::intersectionCheck(Segment * overlap)
{
    /*int x_dist = (min(maxx, overlap->maxx) - max(minx, overlap->minx));
    int y_dist = (min(maxy, overlap->maxy) - max(miny, overlap->miny));
    float areaI = 0;
    float areaOwn = (float) this->size();
    if( x_dist > 0 && y_dist > 0 )
    {
        areaI = x_dist * y_dist;
    }

    
    

    float temp = areaI/areaOwn * 100;*/

    
    return 0;//(int) temp; 
}

void Segment::addRect(Segment * overlap)
{
    maxx = max(maxx,overlap->maxx);
    minx = min(minx,overlap->minx);

    maxy = max(maxy,overlap->maxy);
    miny = min(miny,overlap->miny);
}