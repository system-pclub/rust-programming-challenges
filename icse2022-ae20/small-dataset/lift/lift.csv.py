import csv

with open('lift-data.csv') as fd:
    csvreader = csv.reader(fd)
    headline = csvreader.__next__()
    #print(headline)
    rule2items = {}
    total_cnt = 0
    for i in range(0, len(headline)):
        if 1<=i:
            rule2items[i] = []

    for line in csvreader:
        total_cnt += 1
        for i in range(1, len(line)):
            if line[i] == '1':
                rule2items[i].append(line[0])

    result = {}
    for i in range(6, len(headline)):
        result[headline[i]] = []
        cnt_construct = len(rule2items[i])
        for j in range(1, 6):
            cnt_i_and_j = 0
            cnt_rule = len(rule2items[j])
            for x in rule2items[j]:
                if x in rule2items[i]:
                    cnt_i_and_j += 1
            result[headline[i]].append(cnt_i_and_j * total_cnt / cnt_construct / cnt_rule)
    for key in result:
        print(key.replace(' ', '_').replace('\n', '_'), end=' ')
        for x in result[key]:
            print(x, end=' ')
        print()
