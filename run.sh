#!/usr/bin/env bash
qemu-system-x86_64 -drive format=raw,file=./target/x86_64-ros/debug/bootimage-ros.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04
