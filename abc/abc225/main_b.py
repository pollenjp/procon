from typing import List


def main():

    input_n: int = int(input().rstrip())

    a_list: List[int] = []
    b_list: List[int] = []
    for i in range(input_n - 1):
        a, b = map(int, input().rstrip().split(" "))
        a_list.append(a)
        b_list.append(b)

    mp = {i: [] for i in range(1, input_n + 1)}

    for a, b in zip(a_list, b_list):
        mp[a].append(b)
        mp[b].append(a)

    for v1, list1 in mp.items():
        if len(set(list1)) == input_n - 1:
            print("Yes")
            return
    print("No")


if __name__ == "__main__":
    main()
