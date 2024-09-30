#!/bin/bash
make clean
make -j4
sudo cp libepd.so /usr/local/lib
