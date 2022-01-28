import csv

with open('lift-data.csv') as fd:
    csvreader = csv.reader(fd)
    headline = csvreader.__next__()
    print(headline)
    rule2items = {}

    for i in range(0, len(headline)):
        if 1<=i:
            rule2items[i] = []

    for line in csvreader:
        for i in range(1, len(line)):
            if line[i] == '1':
                rule2items[i].append(line[0])

    