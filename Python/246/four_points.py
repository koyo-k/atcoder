x1, y1 = list(map(int, input().split(' ')))
x2, y2 = list(map(int, input().split(' ')))
x3, y3 = list(map(int, input().split(' ')))
x = sorted([x1, x2, x3])
y = sorted([y1, y2, y3])
if x[0] == x[1]:
    x4 = x[2]
else:
    x4 = x[0]
if y[0] == y[1]:
    y4 = y[2]
else:
    y4 = y[0]
print(x4, y4)
