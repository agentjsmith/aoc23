#!/usr/bin/env python3

import sys
import re

def game_power(line):
    (label,rest) = line.split(": ")

    max_cubes = {}
    reveals = rest.split("; ")
    for reveal in reveals:
        counts = reveal.split(", ")
        for count in counts:
            (n,color) = count.split(" ")
            n = int(n)
            if n > max_cubes.get(color,0):
                max_cubes[color] = n

    return max_cubes.get("red",0) * max_cubes.get("green",0) * max_cubes.get("blue",0)
        
def process(lines):
    sum = 0

    for line in lines:
        sum += game_power(line)

    print(sum)

    	
with open(sys.argv[1],"r") as in_file:
    process([line.rstrip() for line in in_file.readlines()])


