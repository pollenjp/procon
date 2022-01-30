import logging
from dataclasses import dataclass
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


@dataclass
class Val:
    val: int
    pre: "Val" = None
    post: "Val" = None


def main():

    input_n: int = int(input().rstrip())
    input_s: str = input().rstrip()

    val_list = [Val(val=i) for i in range(input_n + 1)]

    start_val: Val = val_list[0]

    for i, c in enumerate(input_s):
        v = val_list[i + 1]
        idx_val = val_list[i]

        if c == "L":
            pre = idx_val.pre
            if pre is not None:
                pre.post = v
                v.pre = pre
            idx_val.pre = v
            v.post = idx_val

            if v.pre is None:
                start_val = v
        else:
            # c == "R"
            post = idx_val.post
            if post is not None:
                post.pre = v
                v.post = post
            idx_val.post = v
            v.pre = idx_val

        # logger.info(
        #     f"{v.val=}, {v.pre.val if v.pre is not None else None =}, {v.post.val if v.post is not None else None =}"
        # )

    # logger.info(f"{start_val.val=}")

    # v = start_val
    # for i in range(6):
    #     logger.info(f"{v.val}")
    #     v = v.post

    v = start_val
    while v is not None:
        print(v.val, end=" ")
        v = v.post


if __name__ == "__main__":
    main()
