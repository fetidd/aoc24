import utils
from collections import defaultdict

class Day5:
	def main(self, data):
		rulesInput, updates = data.split("\n\n")
		# if updates[-1] == "\n":
		# 	updates = updates[:-1]
		
		rules = defaultdict(set)
		for line in rulesInput.split("\n"):
			key, value = line.split("|")
			rules[int(key)].add(int(value))
		# print(rules, updates)
		total = 0
		for line in updates.split("\n"):
			if not line:
				continue
			pages = [int(p) for p in line.split(",")]
			reordered = self.process_update(pages, rules)
			if reordered:
				total += pages[len(pages)//2]
		return total
				
	def process_update(self, pages, rules):
		disallowed = set()
		reordered = False
		i = len(pages) - 1
		while i >= 0:
			page = int(pages[i])
			if page in disallowed:
				reordered = True
				temp = pages[i]
				pages[i] = pages[i+1]
				pages[i+1] = temp
				disallowed.clear()
				i = len(pages) - 1
			else:
				disallowed.update(rules.get(page, []))
				i -= 1
		return reordered
			
