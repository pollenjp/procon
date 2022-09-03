def main():

    input_a, input_b, input_c = list(map(int, input().rstrip().split(" ")))

    for i in range(input_c, input_b + 1, input_c):
        if input_a <= i and i <= input_b:
            print(i)
            return

    print("-1")


if __name__ == "__main__":
    main()
