import utils

class Day1:
    def main(self, data):
        data = data.split("\n")[:-1]
        left, right = [], []
        for line in data:
            l, r = tuple(line.split("   ")[:2])
            left.append(int(l))
            right.append(int(r))
        for l in left, right:
            l.sort()

        print(sum([max(a, b) - min(a, b) for a, b in zip(left, right)]))

        for i in range(len(left)):
            left[i] *= sum([1 for x in right if x == left[i]])
        print(sum(left))
