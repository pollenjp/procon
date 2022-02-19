import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

# prime number under 200
primes_set = set(
    (
        2,
        3,
        5,
        7,
        11,
        13,
        17,
        19,
        23,
        29,
        31,
        37,
        41,
        43,
        47,
        53,
        59,
        61,
        67,
        71,
        73,
        79,
        83,
        89,
        97,
        101,
        103,
        107,
        109,
        113,
        127,
        131,
        137,
        139,
        149,
        151,
        157,
        163,
        167,
        173,
        179,
        181,
        191,
        193,
        197,
        199,
    )
)


def main():

    input_a, input_b, input_c, input_d = map(int, input().rstrip().split())
    for i in range(input_a, input_b + 1):
        can_make_prime: bool = False
        for j in range(input_c, input_d + 1):
            if i + j in primes_set:
                can_make_prime = True
                break
        if not can_make_prime:
            print("Takahashi")
            return
    print("Aoki")


if __name__ == "__main__":
    main()
