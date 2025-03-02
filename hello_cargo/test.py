import os
import hello_cargo
print(dir(hello_cargo.hello_cargo))
from hello_cargo import walk, PyClassIter 
import time
from functools import partial

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
for f in get_all_files(os.walk(path, followlinks=True)):
    res += 1
end = time.time()
print(f"took {end - start}", res)


start = time.time()
res = 0
for f in get_all_files(hello_cargo.PyDirReader(path)):
    res += 1
print("Took", time.time() - start, res)


# res = os.walk("/")
# print(next(res))

