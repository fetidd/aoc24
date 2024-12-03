import pytest
from ..src.day2 import Day2

def test_main():
    data = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"
    assert Day2().main(data.split("\n")) == 2


@pytest.mark.parametrize("report_line, expected", [
    ("7 6 4 2 1", 1),
    ("1 2 7 8 9", 0),
    ("9 7 6 2 1", 0),
    ("1 3 2 4 5", 0),
    ("8 6 4 4 1", 0),
    ("1 3 6 7 9", 1),
])
def test_check_report_line(report_line, expected):
    day = Day2()
    assert day.check_report_line(report_line) == expected
