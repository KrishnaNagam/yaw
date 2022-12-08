#!/usr/bin/python3

import hashlib
import sys

FLAG = "Braindead{this_is_not_the_}"
#Keep playing to find the real flag.

def flip(data, n):
  buflen = len(data)
  while n > buflen:
    n -= buflen
  buf = ["$"] * buflen
  for i in range(buflen):
    try:
      buf[i] = data[i - n]
    except:
      pass
  print(*buf)
  return hashlib.md5("".join(buf).encode("utf-8")).hexdigest()

def check():
    hash_list = [
        "91a7eff3b86a47773ff5b69fdfbb77f3",
        "f2ef1ecce8dae08d2c2a87126d509319",
        "1dd582a46b092569a1498b6e03118aaf",
        "a45966d55b62b59b493782164d36ca19",
        "2fe90b312749c4709a208612e95d9eeb",
        "9f1755cce355b6f999fcf550199f6c92",
        "d9346e8f38b2bc1d2db76fc8699dc258",
        "2d8010c5117c4df9f42a45c876e53eee",
        "a84988c14ee945e03dfbec391a1a0c97",
        "b06b5852e635c7b3cc4757965b54b10e",
        "81d421f3a94c41442c2fd8e02406320a"
    ]
    # real_hash = "76eb685f737828376aa27dfb7b81aad0"
    flag = "pst3R}$$$$$$$$$$$$$$$$$$$$"
    for hash in hash_list:
        for i in range(0, 129):
            if hashlib.md5((chr(i) + flag).encode("utf-8")).hexdigest() == hash:
                flag = chr(i)+flag[:-1]
                print(chr(i))
                break
        print(flag)


if __name__ == "__main__":
    check()
#   print("welcome to fliphash!")
#   print("e.g. flip(\"hello-world!\", 5) =", flip("hello-world!", 5))
#   try:
#     value = int(input("how much do you want to flip? (please enter a digit)\n"))
#     print(flip(FLAG, value))
#   except:
#     pass
