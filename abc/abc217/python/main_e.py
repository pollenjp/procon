import bisect


def main():

    q = int(input().rstrip())

    query = []
    for i in range(q):
        _in = str(input().rstrip())
        query.append(_in)

    a = []

    added_list = []
    q: str
    for q in query:
        if q[0] == "1":
            _, x = list(map(int, q.split()))
            added_list.append(x)
        elif q[0] == "2":
            if len(a) == 0:
                print(added_list.pop(0))
                continue
            print(a.pop(0))
        elif q[0] == "3":
            if len(a) == 0:
                a = sorted(added_list)
            else:
                for x in added_list:
                    bisect.insort(a, x)
            added_list = []
        else:
            raise Exception()


if __name__ == "__main__":
    main()
