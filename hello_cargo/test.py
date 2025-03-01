import os
import hello_cargo
print(dir(hello_cargo.hello_cargo))
from hello_cargo import walk, PyClassIter 
import time

path = "."

def find_path(path):
    for r, dirs, files in os.walk(path):
        for d in dirs:
            yield os.path.join(r, d)
        for f in files:
            yield os.path.join(r, f)


# start = time.time()
# res = 0
# print(len(list(find_path(path))))
# end = time.time()
# print(f"took {end - start}")


# start = time.time()
# print(len(list(hello_cargo.MyIter(path))))
# print("Took", time.time() - start)

print(hello_cargo.read_dir(path))

# res = os.walk("/")
# print(next(res))

