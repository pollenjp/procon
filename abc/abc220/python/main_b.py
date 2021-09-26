def base_n_to_10(x, n):
    out = 0
    for i in range(1, len(str(x)) + 1):
        out += int(x[-i]) * (n ** (i - 1))
    return out


def main():

    input_k: int = int(input().rstrip())
    input_a, input_b = input().rstrip().split(" ")
    ans: int = base_n_to_10(input_a, n=input_k) * base_n_to_10(input_b, n=input_k)
    print(f"{ans}")
    return


if __name__ == "__main__":
    main()
