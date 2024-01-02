CXX=clang++
RM=rm -f

SRCS=main.cc

all: build run run-replace

build:
	$(CXX) -o main $(SRCS)

run:
	./main

run-replace:
	./main --replace

clean:
	$(RM) main
