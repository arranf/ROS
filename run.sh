#!/usr/bin/env bash
qemu-system-x86_64 -drive format=raw,file=./target/x86_64-ros/debug/bootimage-ros.bin
