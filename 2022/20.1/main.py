import sys

def sign(x):
    if x < 0:
        return -1
    if x > 0:
        return +1
    return 0

def main():
    nums = [(int(x.strip()), i) for i, x in enumerate(sys.stdin)]
    for n in nums[:]:
        cur_idx = nums.index(n)
        for o in range(abs(n[0])):
            cur = (cur_idx + o * sign(n[0])) % len(nums)
            next = (cur_idx + (o + 1) * sign(n[0])) % len(nums)
            nums[cur], nums[next] = nums[next], nums[cur]

    idx = [i for i, n in enumerate(nums) if n[0] == 0][0]
    after = [nums[(idx + (i + 1) * 1000) % len(nums)][0] for i in range(3)]
    print(after)
    print(sum(after))

if __name__ == "__main__":
    main()
