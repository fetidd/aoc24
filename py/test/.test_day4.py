import pytest
from day4 import Day4

EXAMPLE = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""

EXAMPLE_REPLACED = """....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"""

EXAMPLE_2_REPLACED = """.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."""

# @pytest.mark.skip("todoben")
def test_main():
	print(EXAMPLE_2_REPLACED)
	assert Day4().main(EXAMPLE_2_REPLACED) == 9

	
@pytest.mark.parametrize(
	"input_grid, expected",
	[
		(".......\n..M.S..\n...A...\n..M.S..\n.......", 1),
		(".......\n..M.M..\n...A...\n..S.S..\n.......", 1),
		(".......\n..S.M..\n...A...\n..S.M..\n.......", 1),
		(".......\n..S.S..\n...A...\n..M.M..\n.......", 1),
		(".......\n..S.S..\n...A...\n..M.M..\n.......", 1),
	]
)
def test_find(input_grid, expected):
	print(input_grid)
	assert expected == Day4().find(input_grid, 7)
