def avg(values):
    return sum(values) / len(values)

def calc_bpm(interval):
    return 60 / interval * 1000

print("st")
st = avg([636, 676, 684, 698])
st_off = avg([121, 127, 113, 120, 135])
print(calc_bpm(st)) # 90
print(st_off / st) # 0.18

print("v1")
v1 = 1000
# v1_off = 500
v1_off = 320
print(calc_bpm(v1)) # 60
print(v1_off / v1) # 0.32

# 60 -> 0.32
# 90 -> 0.18
