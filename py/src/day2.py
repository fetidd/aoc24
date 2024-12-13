from utils import cmp, parse_line

def in_bounds(a, b, min=0, max=3):
    return min < abs(a - b) <= max

class Day2:
    def main(self, data):
        data = data.split("\n")[:-1]
        print(data)
        return sum([self.check_report_line(parse_line(line)) for line in data])

    def check_report_line(self, l):
        if len(set(l)) == 1:
            return 0
        first = [(cmp(l[i], l[i+1]), in_bounds(l[i], l[i+1])) for i in range(len(l)-1)]
        set_first = set(first)
        if len(set_first) == 1 and list(set_first) in ([(1, True)], [(-1, True)]):
            return 1
        for remove in range(len(l)):
            l_copy = l[:]
            l_copy.pop(remove)
            attempt = [(cmp(l_copy[i], l_copy[i+1]), in_bounds(l_copy[i], l_copy[i+1])) for i in range(len(l_copy)-1)]  
            set_attempt = set(attempt)
            if len(set_attempt) == 1 and list(set_attempt) in ([(1, True)], [(-1, True)]):
                return 1
        return 0
                        
