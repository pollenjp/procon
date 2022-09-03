from typing import List


def main():

    s_list: List[str] = []
    s_list.append(input().rstrip())
    s_list.append(input().rstrip())
    s_list.append(input().rstrip())

    t_str: str = input().rstrip()

    target_str: str = ""
    for c in t_str:
        target_str += s_list[int(c) - 1]

    print(target_str)


if __name__ == "__main__":
    main()
