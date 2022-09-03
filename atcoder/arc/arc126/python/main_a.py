import logging
import textwrap
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def solve(cnt_2: int, cnt_3: int, cnt_4: int) -> int:
    logger.info(f"==============={cnt_2=}, {cnt_3=}, {cnt_4=}")
    cnt_answer: int = 0
    cnt_3_x2: int = cnt_3 // 2

    # 4 で 6 を埋める
    if cnt_4 <= cnt_3_x2:
        # 4 が足りない

        cnt_3_x2 -= cnt_4
        cnt_answer += cnt_4
        logger.info(
            textwrap.indent(
                textwrap.dedent(
                    f"""
                    4 が足りない足りない
                    {cnt_3_x2=}
                    {cnt_answer=}
                    """.rstrip()
                ),
                " " * 4,
            )
        )

        cnt_2_x2: int
        cnt_2_x2 = cnt_2 // 2

        # 2x2 で 6 を埋める
        if cnt_2_x2 <= cnt_3_x2:
            # 2x2 が足りない

            cnt_answer += cnt_2_x2
            logger.info(
                textwrap.indent(
                    textwrap.dedent(
                        f"""
                        2 が足りない足りない
                        {cnt_3_x2=}
                        {cnt_answer=}
                        """.rstrip()
                    ),
                    " " * 4,
                )
            )

            return cnt_answer

        else:
            # 2 が足りる

            cnt_answer += cnt_3_x2
            cnt_2 -= cnt_3_x2 * 2  # 使った分を引く
            cnt_answer += cnt_2 // 5
            logger.info(
                textwrap.indent(
                    textwrap.dedent(
                        f"""
                        2 が足りる
                        {cnt_2=}
                        {cnt_answer=}
                        """.rstrip()
                    ),
                    " " * 4,
                )
            )
            return cnt_answer

    # 4 で 6 が埋まった
    cnt_answer += cnt_3_x2
    cnt_4 -= cnt_3_x2
    cnt_4_x2: int = cnt_4 // 2
    cnt_4_left: int = cnt_4 % 2
    logger.info(
        textwrap.indent(
            textwrap.dedent(
                f"""
                4 が足りる
                {cnt_3_x2=}
                {cnt_4_x2=}
                {cnt_answer=}
                """.rstrip()
            ),
            " " * 4,
        )
    )

    # 2 で 4x2 を埋める
    if cnt_2 <= cnt_4_x2:
        # 2 が足りない

        cnt_answer += cnt_2
        logger.info(
            textwrap.indent(
                textwrap.dedent(
                    f"""
                    2 が足りない
                    {cnt_2=}
                    {cnt_answer=}
                    """.rstrip()
                ),
                " " * 4,
            )
        )

        return cnt_answer
    else:
        # 2 が足りる

        cnt_answer += cnt_4_x2
        cnt_2 -= cnt_4_x2
        cnt_answer += (cnt_4_left * 4 + cnt_2 * 2) // 10

        logger.info(
            textwrap.indent(
                textwrap.dedent(
                    f"""
                    2 が足りる
                    {cnt_2=}
                    {cnt_answer=}
                    """.rstrip()
                ),
                " " * 4,
            )
        )

        return cnt_answer


def main():

    input_t = int(input().rstrip())

    cases: List[Tuple[int, int, int]] = []
    for i in range(input_t):
        cases.append(tuple(map(int, input().rstrip().split(" "))))

    for case in cases:
        ans: int = solve(*case)
        print(f"{ans}")


if __name__ == "__main__":
    main()
