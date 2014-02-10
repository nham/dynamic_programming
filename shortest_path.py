graph = {}

def g_ins(i,j,c):
    graph[(i,j)] = c

nodes = 5

g_ins(0, 1, 8)
g_ins(0, 3, 15)
g_ins(1, 3, 3)
g_ins(1, 2, 14)
g_ins(3, 2, 5)
g_ins(2, 4, 10)
g_ins(3, 4, 17)

targets = []
for i in range(0, nodes):
    targets.append( [j for j in range(0, nodes) if (i, j) in graph] )


sources = []
for i in range(0, nodes):
    sources.append( [j for j in range(0, nodes) if (j, i) in graph] )

M = 1000000
v = []
for i in range(0, nodes-1):
    v.append(M)

v.append(0)

def DP1(v, graph, targets):
    keep_going = None
    while keep_going != False:
        print("v = "+str(v))
        newv = []
        for i in range(0, len(v)):
            tmp = None
            for j in targets[i]:
                tmp2 = graph[(i, j)] + v[j]
                if tmp is None or tmp2 < tmp:
                    tmp = tmp2

            if tmp is not None:
                newv.append(tmp)
            else:
                newv.append(v[i])

        keep_going = False
        for i in range(0, len(v)):
            if newv[i] < v[i]:
                keep_going = True

        if keep_going == True:
            v = newv


DP1(v, graph, targets)
