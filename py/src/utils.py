AOC_ROOT = "."

def get_input(day: str):
    data = open(f"{AOC_ROOT}/puzzle_input/day{day}.txt").read()
    if data[-1] == "":
        data.pop(-1)
    return data
    
def parse_line(line, split_char=" ", strip_char=" "):
    return [int(x.strip(strip_char)) for x in line.split(split_char)]

def cmp(a, b):
    return (a > b) - (a < b)
