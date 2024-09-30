#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define BMP_HEADER_SIZE 54
#define IMAGE_WIDTH 1200
#define IMAGE_HEIGHT 825
#define PIXEL_SIZE 3

void skipToImageData(FILE *file) {
    fseek(file, 10, SEEK_SET);
    uint32_t dataOffset;
    fread(&dataOffset, sizeof(uint32_t), 1, file);

    fseek(file, dataOffset, SEEK_SET);
}

int* calculateDiffAABB(const char *filePath1, const char *filePath2) {
    FILE *file1 = fopen(filePath1, "rb");
    FILE *file2 = fopen(filePath2, "rb");

    if (!file1 || !file2) {
        printf("Error opening files.\n");
        return NULL;
    }

    skipToImageData(file1);
    skipToImageData(file2);

    uint8_t pixel1[PIXEL_SIZE];
    uint8_t pixel2[PIXEL_SIZE];

    int minX = IMAGE_WIDTH;
    int minY = IMAGE_HEIGHT;
    int maxX = 0;
    int maxY = 0;

    for (int y = 0; y < IMAGE_HEIGHT; y++) {
        for (int x = 0; x < IMAGE_WIDTH; x++) {
            fread(pixel1, sizeof(uint8_t), PIXEL_SIZE, file1);
            fread(pixel2, sizeof(uint8_t), PIXEL_SIZE, file2);

            int diff = 0;
            for (int i = 0; i < PIXEL_SIZE; i++)
                diff += abs((int)pixel1[i] - (int)pixel2[i]);

            if (diff != 0) {
                if (x < minX) minX = x;
                if (x > maxX) maxX = x;
                if (y < minY) minY = y;
                if (y > maxY) maxY = y;
            }
        }
    }

    fclose(file1);
    fclose(file2);

    int width = maxX - minX + 1;
    int height = maxY - minY + 1;

    int* result = (int*)malloc(4 * sizeof(int));
    result[0] = minX;
    result[1] = minY;
    result[2] = width;
    result[3] = height;

    return result;
}
