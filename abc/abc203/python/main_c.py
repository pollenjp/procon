import sys
from typing import List, Tuple


def solve(n: int, k: int, ab_list: List[Tuple[int, int]]) -> int:
    # sort ab_list
    ab_list = sorted(ab_list, key=lambda x: x[0])

    stock: int = k
    current_place: int = 0
    a: int
    b: int
    for (a, b) in ab_list:
        stock_left: int = stock - (a - current_place)
        if stock_left >= 0:
            stock = stock_left + b
            current_place = a
        else:
            break

    return stock + current_place


if __name__ == "__main__":
    N, K = map(int, sys.stdin.readline().rstrip().split(" "))
    ab_list = []
    for i in range(N):
        a: int
        b: int
        a, b = map(int, sys.stdin.readline().rstrip().split(" "))
        ab_list.append((a, b))
    print(solve(N, K, ab_list))
