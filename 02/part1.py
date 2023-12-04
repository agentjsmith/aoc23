#!/usr/bin/env python3

import sys
import re

def impossible(red,green,blue):
    return (red > 12 or green > 13 or blue > 14)

def check_game(line):
    (label,rest) = line.split(": ")

    # game number
    (_,num) = label.split(" ")
    num = int(num)

    reveals = rest.split("; ")
    for reveal in reveals:
        cubes = {}

        counts = reveal.split(", ")
        for count in counts:
            (n,color) = count.split(" ")
            cubes[color] = int(n)

        if impossible( cubes.get("red",0),
                       cubes.get("green",0),
                       cubes.get("blue",0) ):
            return (num, False)

    return (num, True)
        
def process(lines):
    sum = 0

    for line in lines:
        (num, possible) = check_game(line)
        if possible:
            sum += num

    print(sum)

    	
with open(sys.argv[1],"r") as in_file:
    process([line.rstrip() for line in in_file.readlines()])


