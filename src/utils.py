AOC_ROOT = "."

def get_input(day: str) -> list[str]:
    data = open(f"{AOC_ROOT}/puzzle_input/day{day}.txt")
    return data.readlines()
    
