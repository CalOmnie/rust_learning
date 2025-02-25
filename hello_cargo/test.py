import os
from hello_cargo import walk
import time

path = "/"

start = time.time()
print(walk(path))
end = time.time()
print(f"took {end - start}")


start = time.time()
res = 0
def walky(path):
    for r,dirs,files in os.walk(path):
        for d in dirs:
            yield os.path.join(r, d)
        for f in files:
            yield os.path.join(r, f)



res = os.walk("/")
print(next(res))

