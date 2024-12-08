import utils

class Day4:
	def main(self, data):
		data = data.replace("\n", "") # remove newlines because algorithm won't need them
		return data

	def find(self, grid, line_length=10):
		print(grid)
		count = 0
		for i in range(len(grid)):
			char = grid[i]
			if char == 'X':
				# try to find XMAS
				iM, iA, iS = get_check_indices(i, line_length, len(grid))
				if grid[iM] == "M" and grid[iA] == "A" and grid[iS] == "S":
					count += 1
		return count

	def get_check_indices(index, line_length, grid_length):
		"""
			..........
			...123....
			...4.6....
			...789....
			..........
		"""
		to_try = (
			index - line_length - 1,
			index - line_length,
			index - line_length + 1,
			index - 1,
			index + 1,
			index + line_length - 1,
			index + line_length,
			index + line_length + 1,
		)
		return [i for i in to_try if 0 < i < grid_length]
				
