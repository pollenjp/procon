import bisect 

def main():

    q = int(input().rstrip())

    query = []
    for i in range(q):
        _in = str(input().rstrip())
        query.append(_in)

    a = []

    should_sort_flag: bool = False
    added_list = []
    q: str
    for q in query:
        # print(f"q = {q}")
        if q[0] == "1":
            _, x = list(map(int, q.split()))
            added_list.append(x)
            # bisect.insort(added_list, x) 
        elif q[0] == "2":
            if should_sort_flag:
                for x in added_list:
                    if len(a) == 0:
                        a.append(x)
                    else:
                        bisect.insort(a, x) 
                added_list = []
                should_sort_flag = False
            elif len(a) == 0:
                print(added_list.pop(0))
                continue
            print(a.pop(0))
        elif q[0] == "3":
            should_sort_flag = True
        else:
            raise Exception()


if __name__ == "__main__":
    main()
