#ifndef __EXAMPLE__
#define __EXAMPLE__

#include "../lib/e-Paper/EPD_IT8951.h"
#include "../lib/Config/DEV_Config.h"


// 1 bit per pixel, which is 2 grayscale
#define BitsPerPixel_1 1
// 2 bit per pixel, which is 4 grayscale
#define BitsPerPixel_2 2
// 4 bit per pixel, which is 16 grayscale
#define BitsPerPixel_4 4
// 8 bit per pixel, which is 256 grayscale, but will automatically reduce by hardware to 4bpp, which is 16 grayscale
#define BitsPerPixel_8 8

extern bool Four_Byte_Align;

UBYTE DisplayBmp(const char *imagePath, int x, int y, int width, int height);
void Close();
void Clear();
void Hello();

#endif
