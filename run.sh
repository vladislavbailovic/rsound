#!/bin/bash

if [ -f foo.pcm ]; then
	rm foo.pcm
fi
rustc main.rs && ./main

if [ -f foo.pcm ]; then
	ffplay -autoexit -f f32le -ar 44100 -ac 1 foo.pcm
fi
