#!/usr/bin/env python3

import sys
import re

def process(lines):
    sum = 0

    digit_re = re.compile(r"(\d)")
    for line in lines:
        digits = digit_re.findall(line)
        num = int(digits[0] + digits[-1])
        sum = sum + num

    print(sum)

    	
with open(sys.argv[1],"r") as in_file:
    process([line.rstrip() for line in in_file.readlines()])


