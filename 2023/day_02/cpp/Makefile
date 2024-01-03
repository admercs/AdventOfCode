CXX=clang++
RM=rm -f

SRCS=main.cc

all: build run run-fewest

build:
	$(CXX) -o main $(SRCS)

run:
	./main

run-fewest:
	./main --fewest

clean:
	$(RM) main
