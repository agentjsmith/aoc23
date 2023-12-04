#!/usr/bin/env python3

import sys
import re

lookup = {"one": "1", "two": "2", "three": "3", "four": "4", "five": "5",
          "six": "6", "seven": "7", "eight": "8", "nine": "9" }

pukool = { "".join(reversed(k)): v for k,v in lookup.items() }

first_digit_re = re.compile(r"(\d|" + r"|".join(lookup.keys()) + r")")
last_digit_re = re.compile(r"(\d|" + r"|".join(pukool.keys()) + r")")

print(last_digit_re)

def word_to_digit(word):
    if word in lookup:
        return lookup[word]
    elif word in pukool:
        return pukool[word]
    else:
        return word

def process(lines):
    sum = 0

    for line in lines:
        print(line)
        first = first_digit_re.search(line).group(1)
        last = last_digit_re.search("".join(reversed(line))).group(1)
        print(f"    {first} {last}")
        num = int(word_to_digit(first) + 
                  word_to_digit(last))
        print(f"    {num}")
        sum = sum + num

    print(sum)

    	
with open(sys.argv[1],"r") as in_file:
    process([line.rstrip() for line in in_file.readlines()])


