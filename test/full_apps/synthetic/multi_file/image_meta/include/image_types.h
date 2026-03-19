#ifndef IMAGE_TYPES_H
#define IMAGE_TYPES_H

typedef struct ImageSize {
    unsigned width;
    unsigned height;
} ImageSize;

typedef struct ImageHeader {
    ImageSize size;
    unsigned depth;
} ImageHeader;

#endif
