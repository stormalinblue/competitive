from typing import Self, Union, overload


class ListSlice(object):
    array: list[int]
    sl: range

    def __init__(self, _list: list[int], _slice=None):
        self.array = _list
        if _slice is None:
            self.sl = range(0, len(_list))
        else:
            self.sl = _slice

    def __len__(self):
        dest = min(self.sl.stop, len(self.array)) - self.sl.start
        return dest // self.sl.step

    @overload
    def __getitem__(self, index: int) -> int:
        pass

    @overload
    def __getitem__(self, index: slice) -> Self:
        pass

    def __getitem__(self, index: Union[int, slice]) -> Union[int, Self]:
        if isinstance(index, int):
            # print("indexing sl", self.sl, "with", index, "to get", self.sl[index])
            return self.array[self.sl[index]]
        else:
            # print("before slice", self.sl, "after slice", self.sl[index])
            return self.__class__(self.array, self.sl[index])

    def __str__(self):
        return "[" + ", ".join(str(self.array[x]) for x in self.sl) + "]"


def num_leq(value: int, nums: ListSlice):
    if len(nums) == 0:
        return 0
    elif value >= nums[-1]:
        return len(nums)
    elif value < nums[0]:
        return 0
    else:
        a = 0
        b = len(nums) - 1
        while a <= b:
            m = (a + b) // 2
            # print("abm", a, b, m)
            pivot_value = nums[m]
            next_value = nums[m + 1] if m + 1 < len(nums) else float("inf")

            if pivot_value <= value < next_value:
                return m + 1
            elif pivot_value > value:
                b = m - 1
            else:
                a = m + 1
        raise ValueError


def find_nth(index: int, nums1: ListSlice, nums2: ListSlice) -> int:
    # print("call index", index, nums1, nums2)
    if len(nums1) == 0:
        return nums2[index]
    elif len(nums2) == 0:
        return nums1[index]
    else:
        pivot_index = len(nums1) // 2
        pivot_value = nums1[pivot_index]

        second_index = num_leq(pivot_value, nums2)

        global_pivot_index = pivot_index + second_index
        # print(
        #     "global pivot index of pivot",
        #     pivot_value,
        #     "at",
        #     pivot_index,
        #     "is",
        #     global_pivot_index,
        #     "with second index",
        #     second_index,
        # )

        if index < global_pivot_index:
            return find_nth(index, nums2[:second_index], nums1[:pivot_index])
        elif index > global_pivot_index:
            return find_nth(
                index - global_pivot_index - 1,
                nums2[second_index:],
                nums1[pivot_index + 1 :],
            )
        else:
            return pivot_value


class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        total_length = len(nums1) + len(nums2)

        nums1_slice = ListSlice(nums1)
        nums2_slice = ListSlice(nums2)

        if total_length % 2 == 0:
            first_index = total_length // 2
            second_index = first_index - 1
            return (
                find_nth(first_index, nums1_slice, nums2_slice)
                + find_nth(second_index, nums1_slice, nums2_slice)
            ) / 2
        else:
            return find_nth(total_length // 2, nums1_slice, nums2_slice) / 1


if __name__ == "__main__":
    problems = [
        ([1, 3], [2]),
        (list(range(0, 10, 2)), list(range(1, 10, 2))),
        ([1, 1, 1], [1, 1, 1]),
        (list(range(0, 5)), list(range(5, 7))),
    ]

    solution = Solution()

    for problem in problems:
        print(solution.findMedianSortedArrays(*problem))
