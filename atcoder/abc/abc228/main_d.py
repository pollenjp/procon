import bisect
import collections
import logging
from logging import getLogger
from typing import List, Optional

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

# CONST_INT: int = 10 ** 18 + 1
CONST_INT: int = -1
CONST_N: int = 2 ** 20


class MyDict(collections.abc.MutableMapping):
    def __init__(self, contents):
        "contents must be a sequence of key/value pairs"
        self._list = sorted(contents)

    def __iter__(self):
        return (k for (k, _) in self._list)

    def __setitem__(self, key, value):
        i = bisect.bisect_left(self._list, (key, CONST_INT))
        if i < len(self._list) and self._list[i][0] == key:
            self._list[i] = (key, value)
            return
        self._list.insert(i, (key, value))

    def __delitem__(self, k):
        i = bisect.bisect_left(self._list, (k, CONST_INT))
        if not (i < len(self._list) and self._list[i][0] == k):
            raise Exception()
        del self._list[i]

    def __contains__(self, k):
        i = bisect.bisect_left(self._list, (k, CONST_INT))
        logger.info(f"{i=}")
        return i < len(self._list) and self._list[i][0] == k

    def __len__(self):
        return len(self._list)

    def __getitem__(self, k):
        i = bisect.bisect_left(self._list, (k, CONST_INT))
        if i >= len(self._list):
            raise KeyError(k)
        return self._list[i][1]

    def next(self, k):
        """次の要素のキーを返す"""
        # k は key に含まれていることを前提とする.
        i = bisect.bisect_left(self._list, (k, CONST_INT))
        assert self._list[i][0] == k
        end_idx = (i + 1) % len(self._list)
        return (self._list[i][0] + 1, self._list[end_idx][0])


def main():
    input_q: int = int(input().rstrip())

    # ここにキーが存在する == 要素が -1 以外である
    binary_dict = MyDict([])

    logger.info(f"{binary_dict=}")

    for _ in range(input_q):
        t, x = map(int, input().rstrip().split())
        logger.info(f"{t=} {x=}")
        if t == 1:
            h = x
            h %= CONST_N
            if not binary_dict.__contains__(h):
                binary_dict[h] = x
            else:
                h, end = binary_dict.next(h)
                while True:
                    h %= CONST_N
                    if h == end:
                        h, end = binary_dict.next(h)
                        continue
                    if not binary_dict.__contains__(h):
                        binary_dict[h] = x
                        break
                    h += 1
            logger.info(f"{h=}, {x=}")
        else:  # t == 2
            x %= CONST_N
            logger.info(f"{x=}")
            if binary_dict.__contains__(x):
                print(f"{binary_dict[x]}")
            else:
                print("-1")


if __name__ == "__main__":
    main()
