DEBUG := -g

HDRS := $(shell find . -name *.hpp)
SRCS := $(shell find . -name *.cpp)

.DEFAULT_GOAL := run-fresh

deps:
	sudo apt install libncurses5-dev libncursesw5-dev

build: $(SRCS) $(HDRS)
	g++ -std=c++2a $(DEBUG) $(SRCS) -Isrc -lncurses -o ./ccg

run:
	./ccg

run-fresh: build run
