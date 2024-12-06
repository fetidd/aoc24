AOC_ROOT = "."

def get_input(day: str) -> list[str]:
    data = open(f"{AOC_ROOT}/puzzle_input/day{day}.txt")
    return data.readlines()
    
def parse_line(line, split_char=" ", strip_char=" "):
    return [int(x.strip(strip_char)) for x in line.split(split_char)]

def cmp(a, b):
    print(f"cmp {a} {b}")
    return (a > b) - (a < b)
