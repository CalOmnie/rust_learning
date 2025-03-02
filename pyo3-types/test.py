import os
import pyo3_types as pyo3
print(dir(pyo3))
import time

def get_all_files(iterator):
    for r, dirs, files in iterator:
        yield r
        for d in dirs:
            yield d
        for f in files:
            yield f

path = "/home/louis"
start = time.time()
res = 0

for f in get_all_files(pyo3.DirReader(path)):
    res += 1
print("Took", time.time() - start, res)


start = time.time()
res = 0
for f in get_all_files(os.walk(path, followlinks=True)):
    res += 1
end = time.time()
print(f"took {end - start}", res)



test = pyo3.DirReader(path)
print(next(test))

test2 = os.walk(path)
print(next(test2))

# res = os.walk("/")
# print(next(res))

