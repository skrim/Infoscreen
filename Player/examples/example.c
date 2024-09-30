#include "example.h"

#include <time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdlib.h>
#include <signal.h>

#include "findDiffAABB.h"
#include "../lib/e-Paper/EPD_IT8951.h"
#include "../lib/GUI/GUI_Paint.h"
#include "../lib/GUI/GUI_BMPfile.h"
#include "../lib/Config/Debug.h"

bool Four_Byte_Align = false;

extern int epd_mode;
extern UWORD VCOM;
extern UBYTE isColor;


void measure_time(clock_t start_time, const char *message) {
    clock_t end_time = clock(); // Capture the ending time

    // Calculate the elapsed time in milliseconds
    double elapsed_time = (double)(end_time - start_time) * 1000 / CLOCKS_PER_SEC;

    printf("%s at %.2f milliseconds\n", message, elapsed_time);
}

UWORD VCOM = 1710;
int epd_mode = 0;

bool Initialized = false;
IT8951_Dev_Info Dev_Info = {0, 0};
UWORD Panel_Width;
UWORD Panel_Height;
UDOUBLE Init_Target_Memory_Addr;

void Handler(int signo){
    Debug("\r\nHandler:exit\r\n");
	if(Dev_Info.Panel_W != 0){
		Debug("Going to sleep\r\n");
		EPD_IT8951_Sleep();
	}
    DEV_Module_Exit();
    exit(0);
}

UBYTE Initialize() {
    if (Initialized)
        return 0;

    signal(SIGINT, Handler);

    Debug("Module init\n");

    if (DEV_Module_Init() != 0) {
        Debug("Initialization failed!\n");
        return -1;
    }

    Debug("Getting dev info, VCOM %d\n", VCOM);

    Dev_Info = EPD_IT8951_Init(VCOM);

    Debug("Received dev info\n");

    Panel_Width = Dev_Info.Panel_W;
    Panel_Height = Dev_Info.Panel_H;
    Init_Target_Memory_Addr = Dev_Info.Memory_Addr_L | (Dev_Info.Memory_Addr_H << 16);

    Debug("- Init_Target_Memory_Addr: %d\n", Init_Target_Memory_Addr);

    Debug("- Panel width: %d, height: %d\n", Panel_Width, Panel_Height);
    Debug("- Memory: low %u, high %u\n", Dev_Info.Memory_Addr_L, Dev_Info.Memory_Addr_H);

    Initialized = true;
    return 0;
}

UBYTE DisplayBmp(const char *imagePath, int x, int y, int width, int height) {
    Debug("Displaying %s\n", imagePath);
    if (x + width > 1200 || y + height > 825 || x < 0 || y < 0) {
        Debug("INVALID IMAGE AREA: x %d, y %d, w %d, h %d\n", x, y, width, height);
        return -1;
    }

    Debug("Start initialize\n");

    Initialize();

    Debug("Initialize complete\n");

    UBYTE bpp = BitsPerPixel_4;
    UDOUBLE Imagesize = ((Panel_Width * bpp % 8 == 0)? (Panel_Width * bpp / 8 ): (Panel_Width * bpp / 8 + 1)) * Panel_Height;
    UBYTE *Refresh_Frame_Buf = (UBYTE *)malloc(Imagesize);

    if(Refresh_Frame_Buf == NULL) {
        Debug("Failed to apply for black memory...\r\n");
        return -1;
    }

    Paint_NewImage(Refresh_Frame_Buf, width, height, 0, BLACK);
    Paint_SetBitsPerPixel(bpp);
    Paint_Clear(WHITE);

    if (GUI_ReadBmp(imagePath, 0, 0) != 0) {
        Debug("Failed to read file %s\n", imagePath);
        return -1;
    }

    Debug("Refreshing screen...\n");

    EPD_IT8951_4bp_Refresh(Refresh_Frame_Buf, x, y, width, height, false, Init_Target_Memory_Addr, false);

    if(Refresh_Frame_Buf != NULL){
        free(Refresh_Frame_Buf);
        Refresh_Frame_Buf = NULL;
    }

    return 0;
}

void Clear() {
    Initialize();
    EPD_IT8951_Clear_Refresh(Dev_Info, Init_Target_Memory_Addr, INIT_Mode);
}

void Close() {
    Debug("Module_Exit()\n");
    DEV_Module_Exit();
}

void Hello() {
    DisplayBmp("/home/matias/testimages/1200x825_3.bmp", 0, 0, 1200, 825);
    DisplayBmp("/home/matias/testimages/cropped.bmp", 100, 100, 200, 200);
    DisplayBmp("/home/matias/testimages/cropped.bmp", 400, 400, 200, 200);
    DisplayBmp("/home/matias/testimages/1200x825_1.bmp", 0, 0, 1200, 825);
    DisplayBmp("/home/matias/testimages/screenshot.bmp", 0, 0, 1200, 825);
    DEV_Delay_ms(1000);
}

int main(int argc, char *argv[])
{
    //Hello();
}
