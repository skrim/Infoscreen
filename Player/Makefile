DIR_Config   = ./lib/Config
DIR_EPD      = ./lib/e-Paper
DIR_FONTS    = ./lib/Fonts
DIR_GUI      = ./lib/GUI
DIR_Examples = ./examples

DIR_BIN      = ./bin

OBJ_C = $(wildcard ${DIR_Config}/*.c ${DIR_EPD}/*.c ${DIR_FONTS}/*.c ${DIR_GUI}/*.c ${DIR_Examples}/*.c )
OBJ_O = $(patsubst %.c,${DIR_BIN}/%.o,$(notdir ${OBJ_C}))

TARGET = libepd.so

CC = gcc

MSG = -g -O0 -Wall
DEBUG = -D USE_DEBUG
STD = -std=gnu99

CFLAGS += $(MSG) $(DEBUG) $(STD) -fPIC

LIB = -lbcm2835 -lm -lrt -lpthread

$(shell mkdir -p $(DIR_BIN))

${TARGET}:${OBJ_O}
	$(CC) $(CFLAGS) -shared -o $@ $(OBJ_O) $(LIB)

${DIR_BIN}/%.o:$(DIR_Config)/%.c
	$(CC) $(CFLAGS) -c  $< -o $@

${DIR_BIN}/%.o:$(DIR_EPD)/%.c
	$(CC) $(CFLAGS) -c  $< -o $@

${DIR_BIN}/%.o:$(DIR_FONTS)/%.c
	$(CC) $(CFLAGS) -c  $< -o $@

${DIR_BIN}/%.o:$(DIR_GUI)/%.c
	$(CC) $(CFLAGS) -c  $< -o $@

${DIR_BIN}/%.o:$(DIR_Examples)/%.c
	$(CC) $(CFLAGS) -c  $< -o $@

clean :
	rm $(DIR_BIN)/*.*
	rm $(TARGET)

