import pytest
from day6 import Day6

EXAMPLE = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""

SOLVED = """....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."""


def test_main():
	# assert Day6().main(EXAMPLE) == 41 # the number of Xs in SOLVED
	assert Day6().main(EXAMPLE) ==  6 # the number of places a new obstacle could be placed to induce a loop


@pytest.mark.parametrize(
	"grid, expected",
	[
		(".....\n..<..\n.....\n.....\n.....", [("<", (2, 1))]),
		(".....\n..<..\n.....\n...^.\n.....", [("<", (2, 1)), ("^", (3, 3))]),
		("....<\n..<..\n.....\n...^.\n>....", [("<", (4, 0)), ("<", (2, 1)), (">", (0, 4)), ("^", (3, 3))]),
	]
)
def test_find_guards(grid, expected):
	d = Day6()
	d.COLS = 5
	d.ROWS = 5
	d.GRID = grid
	assert expected == d.find_guards()
