def main():

    p_list = list(map(int, input().rstrip().split(" ")))

    alphabet_str = "abcdefghijklmnopqrstuvwxyz"

    target_str = ""
    for p_i in p_list:
        target_str += alphabet_str[p_i - 1]

    print(target_str)


if __name__ == "__main__":
    main()
