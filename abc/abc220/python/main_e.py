import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
# logger.setLevel(logging.INFO)
logger.setLevel(logging.WARNING)


CONST_MOD_VAL: int = 998244353


def main():
    input_n, input_d = map(int, input().rstrip().split())

    exp_2_divided_list = [1]
    val: int = 1
    for i in range(1, max(input_n, input_d)):
        val *= 2
        exp_2_divided_list.append(val % CONST_MOD_VAL)

    def f(depth: int, n: int, target_length: int):
        # 各頂点 v について、「距離が D である頂点の組 (i,j)であって、i,j のLCA (Lowest Common Ancestor) が vであるようなもの」の個数
        # を考え、これを f(v) とおきます。

        cnt: int = 0

        # v=i または v=j の場合
        if depth + target_length <= n - 1:
            cnt += exp_2_divided_list[target_length]
            cnt %= CONST_MOD_VAL

        logger.info(f"{cnt=}")

        # if target_length <= n - depth - 1:
        #     return (
        #         (exp_2_divided_list[target_length - 2]) * (target_length - 1) + exp_2_divided_list[target_length]
        #     ) % CONST_MOD_VAL
        # if 2 * (n - depth - 1) >= target_length > n - depth - 1:
        #     return (
        #         exp_2_divided_list[target_length - 2] * (target_length - 1 - 2 * (target_length - n - depth))
        #     ) % CONST_MOD_VAL
        # return 0

        # それ以外の場合
        d_max: int = (n - 1) - depth
        logger.info(f"{d_max=}")
        if (target_length - 1) <= d_max:
            cnt += ((target_length - 1) * (exp_2_divided_list[target_length - 2])) % CONST_MOD_VAL  # 実際は計算量節約すべし
            logger.info(f"{cnt=}, {d_max=}")
            cnt %= CONST_MOD_VAL
        else:
            k_max: int = d_max
            k_min: int = target_length - k_max
            num: int = max(0, k_max - k_min + 1)
            if num > 0:
                logger.info(f"{k_max=}, {k_min=}, {num=}")
                cnt += (num * (exp_2_divided_list[target_length - 2])) % CONST_MOD_VAL
                cnt %= CONST_MOD_VAL
        logger.info(f"{cnt=}")

        return cnt

    # forloop by depth
    cnt_answer: int = 0
    for depth in range(input_n):
        # logger.info(f"{depth=}, {f(depth, input_n, input_d)=}")
        cnt_answer += (exp_2_divided_list[depth] * f(depth, input_n, input_d)) % CONST_MOD_VAL
        cnt_answer %= CONST_MOD_VAL

    # (i, j) and (j, i)
    cnt_answer *= 2
    cnt_answer %= CONST_MOD_VAL

    print(f"{cnt_answer}")


if __name__ == "__main__":
    main()
