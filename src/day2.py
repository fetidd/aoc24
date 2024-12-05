from utils import cmp, parse_line

class Day2:
    def main(self, data):
        return sum([self.check_report_line(parse_line(line)) for line in data])

    def check_report_line(self, line):
        print(line)
    

def in_bounds(a, b, min=0, max=3):
    return min < abs(a - b) <= max

