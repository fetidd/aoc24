#! /usr/bin/env python
import sys
import importlib
from utils import get_input
day_to_run = sys.argv[1]
importlib.import_module(f"day{day_to_run}")
day = getattr(sys.modules[f"day{day_to_run}"], f"Day{day_to_run}")()
data = get_input(int(day_to_run))
day.main(data)
