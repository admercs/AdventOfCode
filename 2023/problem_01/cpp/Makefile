CXX=clang++
RM=rm -f
#CPPFLAGS, LDFLAGS, LDLIBS

SRCS=main.cc
#OBJS

all: build run

build:
#    $(CXX) $(CPPFLAGS) $(LDFLAGS) -o tool $(OBJS) $(LDLIBS)
	$(CXX) -o main.o $(SRCS)

run:
	./main.o

clean:
	$(RM) main.o
