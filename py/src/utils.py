from pathlib import Path

def get_input(day: str):
    data = open(Path.home() / f"aoc24/puzzle_input/day{day}.txt").read().rstrip()
    return data
    
def parse_line(line, split_char=" ", strip_char=" "):
    return [int(x.strip(strip_char)) for x in line.split(split_char)]

def cmp(a, b):
    return (a > b) - (a < b)
