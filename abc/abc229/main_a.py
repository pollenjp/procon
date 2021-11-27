import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s1 = input().rstrip()
    input_s2 = input().rstrip()

    if input_s1 == "##" or input_s1 == "..":
        print("Yes")
        return

    if input_s2 == "##" or input_s1 == "..":
        print("Yes")
        return

    s1_0: bool = input_s1[0] == "#"
    s2_0: bool = input_s2[0] == "#"

    # logger.info(f"{s1_0} {s2_0}")
    if s1_0 is not s2_0:
        print("No")
        return
    print("Yes")


if __name__ == "__main__":
    main()
