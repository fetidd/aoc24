import utils
import re

MUL = r"mul(\([0-9]{1,3},[0-9]{1,3}\))"
DO = r"(do\(\))"
DONT = r"(don't\(\))"

REGEX = re.compile(r"%s|%s|%s"%(MUL, DO, DONT))

class Day3:
	def main(self, data):
		muls = self.find_muls(data)
		print(muls)
		return sum([x[0]*x[1] for x in muls])
		
	def find_muls(self, input_string):
		tokens = REGEX.findall(input_string)
		muls = []
		take = True
		for mul, do, dont in tokens:
			print(mul, do, dont)
			if dont:
				take = False
			elif do:
				take = True
			elif take and mul:
				print("take", mul)
				muls.append(eval(mul))
		return muls
