import os
import hello_cargo
import time
from functools import partial

def get_all_files(iterator):
    for r, dirs, files in iterator:
        for d in dirs:
            yield d
        for f in files:
            yield f

path = "/"

start = time.time()
res = 0
for f in get_all_files(os.walk(path)):
    res += 1
end = time.time()
print(f"Base os.walk took {end - start}", res)


start = time.time()
res = 0
for f in get_all_files(hello_cargo.PyDirReader(path)):
    res += 1
print("My cool AF rust implementation Took", time.time() - start, res)


# res = os.walk("/")
# print(next(res))

