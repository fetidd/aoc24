import pytest
from ..src.day3 import Day3

def test_main():
	assert Day3().main(["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]) == 48

@pytest.mark.parametrize(
	"input_string, expected",
	[
		("mul(1,1)", [(1, 1),]),
		("mul(1,)", []),
		("mul(1,1)mul(2,2)", [(1, 1), (2, 2)]),
		("mul(11,11)mul(2,2)", [(11, 11), (2, 2)]),
		("mul(111,111)mul(2,2)", [(111, 111), (2, 2)]),
		("mul(1111,1111)mul(2,2)", [(2, 2)]),
		("mul(1,1)mul(2,2)", [(1, 1), (2, 2)]),
		("do_not_mul(1,1)mul(2,2)", [(1, 1), (2, 2)]),
		("mul(1,1) mul(2,2)", [(1, 1), (2, 2)]),
		("mul(1,1)_$^ mul(2,2)", [(1, 1), (2, 2)]),
		("&*mul(1,1)_$^ mul(2,2)", [(1, 1), (2, 2)]),
		("mul(1, 1)mul(2,2)", [(2, 2)]),
		("mul( 1,1)mul(2,2)", [(2, 2)]),
		("mul(1 ,1)mul(2,2)", [(2, 2)]),
		("mul(1,1 )mul(2,2)", [(2, 2)]),
		("mul[1,1]mul(2,2)", [(2, 2)]),
		
		("mul(1,1)don't()mul(2,2)", [(1, 1),]),
		("mul(1,1)fdon't()mul(2,2)willdo()mul(3,3)", [(1, 1), (3, 3)]),
	]
)
def test_find_muls(input_string, expected):
	assert expected == Day3().find_muls(input_string)
