#!/bin/sh
cargo b || exit 1
sudo setcap cap_sys_rawio+ep target/debug/infoscreen || exit 2
target/debug/infoscreen || exit 3
