from utils import cmp, parse_line

def in_bounds(a, b, min=0, max=3):
    return min < abs(a - b) <= max

class Day2:
    def main(self, data):
        return sum([self.check_report_line(parse_line(line)) for line in data])

    def check_report_line(self, l):
        skipped = False
        trend = 0
        prev = l[0]
        for i, curr in enumerate(l[1:]):
            safe = in_bounds(prev, curr)
            if safe:
                curr_trend = cmp(prev, curr)
                if not curr_trend: # the levels are equal, so skip
                    if skipped:
                        return 0 # report unsafe if already skipped
                    skipped = True
                else:
                    trend = curr_trend
                prev = curr
                
            else: # we need to determine which of the two values to skip
                pass
        return 1
                
       
