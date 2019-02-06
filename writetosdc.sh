#!/usr/bin/env bash

./build.sh
dd if=./target/x86_64-ros/debug/bootimage-ros.bin of=/dev/sdc && sync