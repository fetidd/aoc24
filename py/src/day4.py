import utils

class Day4:
	def main(self, data):
		line_length = data.index("\n")
		return self.find(data, line_length=line_length)

	def find(self, grid, line_length=10):
		count = 0
		for i in range(line_length + 1, len(grid) - line_length):
			column = i % (line_length + 1)
			if 0 < column < line_length - 1:
				if grid[i] == 'A':
					NW = i - line_length - 2
					NE = i - line_length
					SW = i + line_length
					SE = i + line_length + 2
					valids = (("M", "S"), ("S", "M"))
					if (grid[NW], grid[SE]) in valids and (grid[NE], grid[SW]) in valids:
						count += 1
		return count

				
