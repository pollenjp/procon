def main():

    input_a, input_b = list(map(int, input().rstrip().split(" ")))

    ans = 1
    for i in range(input_a - input_b):
        ans *= 32

    print(f"{ans}")


if __name__ == "__main__":
    main()
