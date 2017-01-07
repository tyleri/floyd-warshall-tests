import sys

s = ""

while True:
    try:
        s += input() + "\n"
    except Exception:
        f = open(sys.argv[1])
        print(s[:-1] == f.read())
        f.close()
        exit()
