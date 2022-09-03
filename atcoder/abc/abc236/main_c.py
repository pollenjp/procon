import logging
from logging import getLogger
from typing import Dict, List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n, input_m = map(int, input().split())
    input_s_list: List[str] = input().rstrip().split()
    input_t_list: List[str] = input().rstrip().split()

    station_can_stop_dict: Dict[str, bool] = {s: False for s in input_s_list}
    for s in input_t_list:
        station_can_stop_dict[s] = True

    for s in input_s_list:
        if station_can_stop_dict[s]:
            print("Yes")
            continue

        print("No")


if __name__ == "__main__":
    main()
