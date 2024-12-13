import utils

class Day6:
	ROWS = None
	COLS = None
	GRID = None
	
	def main(self, data):
		Day6.GRID = data.rstrip()
		Day6.COLS = data.index("\n")
		Day6.ROWS = len(data) // self.COLS
		print(f"Analysing {self.COLS} x {self.ROWS} grid...")
		guards = self.find_guards()
		for guard in guards:
			

	def find_guards(self):
		guards = []
		for guard in ("<", ">", "v", "^"):
			for i in range(len(self.GRID) - 1):
				if self.GRID[i] == guard:
					print(i)
					x = i % (self.COLS + 1)
					y = i // (self.ROWS + 1)
					print(f"{i} % {self.COLS + 1} = {x}")
					print(f"{i} // {self.ROWS + 1} = {y}")
					guards.append((guard, (x, y)))
		return guards

	class Guard:
		def __init__(self, starting_dir, start_point):
			self.dir = starting_dir
			self.x = start_point[0]
			self.y = start_point[1]

		def move(self):
			if self.dir == "^":
				self.y -= Day6.ROWS - 1
			elif self.dir == ">":
				self.x += 1
			elif self.dir == "<":
				self.x -= 1
			elif self.dir == "v":
				self.y += Day6.ROWS + 1
			return self.check_bounds(), (self.x, self.y)

		def check_bounds(self):
			if 0 > self.x or Day6.COLS - 1 < self.x:
				return False
			if 0 > self.y or Day6.ROWS - 1 < self.y:
				return False
			return True
				
			
