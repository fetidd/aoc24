AOC_ROOT = "."

def test_input(day: str) -> list[str]:
    data = open(f"{AOC_ROOT}/day{day}.txt")
    return data.readlines()
    
