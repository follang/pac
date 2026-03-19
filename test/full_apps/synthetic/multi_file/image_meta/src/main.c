#include "image_meta.h"

unsigned image_pixels(const ImageHeader *header)
{
    return header->size.width * header->size.height;
}

int main(void)
{
    ImageHeader header = { { 8u, 8u }, 24u };
    return image_pixels(&header) == 64u ? 0 : 1;
}
