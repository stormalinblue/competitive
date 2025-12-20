def geq_search(array: list[int], slc: range, target: int) -> int:
    print("looking for minimum index >", target, "in", array, slc, "otherwise len")

    a = slc[0]
    b = slc[-1]

    while b >= a:
        m = (a + b) // 2
        print("a is", a, "b is", b, "m is", m)

        print(array[m], target, array[m + 1] if m + 1 < slc.stop else "Done")

        if array[m] > target:
            b = m - 1
        elif array[m] == target:
            if m + 1 < slc.stop:
                if array[m + 1] > target:
                    return m + 1
                else:
                    a = m + 1
            else:
                return m + 1
        else:  # target > array[m]
            if m + 1 < slc.stop:
                if array[m + 1] > target:
                    return m + 1
                else:
                    a = m + 1
            else:
                return m + 1
    return len(slc)


def find_nth(
    index: int,
    nums1: list[int],
    nums1_slice: range,
    nums2: list[int],
    nums2_slice: range,
):
    print("call find nth", index, nums1, nums1_slice, nums2, nums2_slice)
    if len(nums1_slice) == 0:
        print(nums1_slice, nums2_slice, "degen one")
        return nums2[nums2_slice[index]]
    elif len(nums2_slice) == 0:
        print(nums1_slice, nums2_slice, "degen two")
        return nums1[nums1_slice[index]]
    else:
        pivot_index = len(nums1_slice) // 2
        pivot = nums1[nums1_slice[pivot_index]]

        second_index = geq_search(nums2, nums2_slice, pivot)
        print("determined that pivot is at", second_index, "relative second slice")

        print(nums1_slice, nums2_slice, second_index)

        k = second_index + pivot_index
        print("determined that pivot is at global slice index", k)
        if index == k:
            print(nums1_slice, nums2_slice, "found")
            return pivot
        elif index > k:
            print(nums1_slice, nums2_slice, "greater")
            return find_nth(
                index - k - 1,
                nums2,
                nums2_slice[second_index + 1 :],
                nums1,
                nums1_slice[pivot_index + 1 :],
            )
        else:
            print(nums1_slice, nums2_slice, "lesser")
            return find_nth(
                index,
                nums2,
                nums2_slice[:second_index],
                nums1,
                nums1_slice[:pivot_index],
            )


class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        result = []
        for i in [4, 5]:  # range(len(nums1) + len(nums2)):
            result.append(
                find_nth(i, nums1, range(len(nums1)), nums2, range(len(nums2)))
            )
            print(result[-1])
        print(result)


if __name__ == "__main__":
    problems = [
        # ([1, 3], [2]),
        (list(range(0, 10, 2)), list(range(1, 10, 2)))
    ]

    solution = Solution()

    for problem in problems:
        solution.findMedianSortedArrays(*problem)
