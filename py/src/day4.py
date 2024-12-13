import utils

TARGET = "MAS"

class Day4:
	def main(self, data):
		line_length = data.index("\n")
		return self.find(data, line_length=line_length)

	def find(self, grid, line_length=10):
		# print(grid)
		count = 0
		for i in range(len(grid)):
			if grid[i] == 'X':
				# try to find XMAS
				# print(f"searching from {i}...")
				multipliers = (
					line_length,
					line_length + 1,
					line_length + 2,
					1,
				)
				for multiplier in multipliers:
					for mod in (lambda i, f: i - f, lambda i, f: i + f):
						found = True
						for x in range(3):
							search_index = mod(i, ((x + 1) * multiplier))
							# print(f"\t@{search_index}", end="")
							if 0 <= search_index < len(grid):
								if grid[search_index] != TARGET[x]:
									# print("but couldn't find it!")
									# print("")
									found = False
									break
								# print(f" and found {TARGET[x]}")
							else:
								# print("but went off the end of the grid!")
								# print("")
								found = False
								break
						if found:
							# print("\tfound XMAS!")
							count += 1
		return count

				
