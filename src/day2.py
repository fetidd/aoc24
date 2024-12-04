import utils

class Day2:
    def main(self, data):
        return sum([self.check_report_line([int(x) for x in line.split(" ")]) for line in data])

    def check_report_line(self, line):
        prev_change = cmp(line[0], line[1])
        if prev_change == 0:
            skipped = True
            print(" and skipped!")
            prev_change = cmp(line[1], line[2])
            if prev_change == 0:
                print(" and found a second fail before loop")
                return 0
            l = 2
            r = 3
        else:
            skipped = False
            print("")
            l = 1
            r = 2
        print("start loop")
        while r < len(line):
            print(f"l={l} r={r}")
            change = cmp(line[l], line[r])
            if change != prev_change:
                if not skipped:
                    print(" and skipped!")
                    r += 1
                    skipped = True
                    continue
                print(" and failed")
                return 0
            print("")
            prev_change = change
            l += 1
            r += 1
        return 1

    
def cmp(a, b):
    print(f"comparing {a} and {b}", end="")    
    return (a > b) - (a < b) if 0 < abs(a - b) <= 3 else 0
