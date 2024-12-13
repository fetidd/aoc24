import pytest
from ..src.day4 import Day4

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

# @pytest.mark.skip("todoben")
def test_main():
	assert Day4().main(EXAMPLE) == 18

	
@pytest.mark.parametrize(
	"input_grid, expected",
	[
		(
".XMAS..S..\n\
.M..A.A...\n\
.A..MM..XM\n\
AS..XMAS..\n\
....SAMX..", 6),
		(
"AXMASXMSMX\n\
.M..A.A...\n\
.A..MM..XM\n\
AS..XMAS..\n\
....SAMX..", 6),
	]
)
def test_find(input_grid, expected):
	assert expected == Day4().find(input_grid)
