def main():

    input_x = int(input().rstrip())

    if 0 <= input_x and input_x < 40:
        next_rank_boarder = 40
    elif 40 <= input_x and input_x < 70:
        next_rank_boarder = 70
    elif 70 <= input_x and input_x < 90:
        next_rank_boarder = 90
    elif 90 <= input_x:
        print("expert")
        return
    else:
        raise Exception("input_x is out of range")
    print(f"{next_rank_boarder - input_x}")


if __name__ == "__main__":
    main()
