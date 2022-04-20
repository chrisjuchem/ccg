SRCS := ./src/*.cpp

.DEFAULT_GOAL := run-fresh

deps:
	sudo apt install libncurses5-dev libncursesw5-dev

build: $(SRCS)
	g++ $(SRCS) -lncurses -o ./build/ccg 

run:
	./build/ccg

run-fresh: build run
