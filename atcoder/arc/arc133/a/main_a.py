import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n = int(input())
    a_list = list(map(int, input().rstrip().split()))

    remove_idx: int = len(a_list) - 1
    for i in range(0, len(a_list) - 1):
        if a_list[i] > a_list[i + 1]:
            remove_idx = i
            break

    new_list = []
    for val in a_list:
        if val != a_list[remove_idx]:
            new_list.append(val)

    print(f"{' '.join(map(str, new_list))}")


if __name__ == "__main__":
    main()
