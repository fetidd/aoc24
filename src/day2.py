import utils

class Day2:
    def main(self, data):
        return sum([self.check_report_line(line) for line in data])
            

    def check_report_line(self, line):
        line = [int(x) for x in line.split(" ")]
        for i, report in enumerate(line):
            pass
        return 1

