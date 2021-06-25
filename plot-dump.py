import sys
import matplotlib.pyplot as plt

with open(sys.argv[1], "r") as fd:
	names = list(map(str.strip, fd.readline().split(",")))
	values = {}
	for name in names:
		values[name] = []

	for line in fd.readlines():
		for i, val in enumerate(line.split(",")):
			if val == "" or val == "\n":
				continue

			val = float(val)
			values[names[i]].append(val)

for name in values:
	vals = values[name]

	fig, ax = plt.subplots()
	ax.plot(list(range(len(vals))), vals)

	ax.set(xlabel='time', ylabel=name)
	ax.grid()

	filename = "plots/" + name + ".png"
	fig.savefig(filename, dpi=600)
