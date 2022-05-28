main: main.rs
	rustc main.rs

foo.pcm: main
	./main

foo.ppm: main
	./main

.PHONY:image
image: foo.ppm
	feh foo.ppm

.PHONY:sound
sound: foo.pcm
	ffplay -autoexit -f f32le -ar 44100 -ac 1 foo.pcm
