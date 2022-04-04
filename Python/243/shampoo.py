v, a, b, c = list(map(int, input().split(' ')))
s = a + b+ c
if (v % s < a):
    print('F')
elif (v % s < (a + b)):
    print('M')
else:
    print('T')