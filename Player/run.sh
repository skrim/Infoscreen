#!/bin/sh
sudo make clean
sudo make -j4

sudo UPDATE_SCRIPT_PATH="$(pwd)/update_image.sh" IMAGE_PATH="/ramdisk/screenshot.bmp" PREVIOUS_IMAGE_PATH="/ramdisk/screenshot.previous.bmp" ./epd -1.71 0
