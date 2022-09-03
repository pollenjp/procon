def main():

    input_n = int(input().rstrip())
    input_s = str(input().rstrip())

    s = input_s[input_n - 1]
    if s == "o":
        print("Yes")
    elif s == "x":
        print("No")
    else:
        raise Exception()


if __name__ == "__main__":
    main()
