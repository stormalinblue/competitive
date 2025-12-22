class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        active_rectangles = {}
        heights.append(0)

        max_area = 0

        height_stack = [(0, -1)]

        for index, height in enumerate(heights):
            # print(height_stack)
            last_height, last_index = height_stack[-1]

            if height >= last_height:
                height_stack.append((height, index))
            else:
                last_index = len(heights)
                while True:
                    top_height, top_index = height_stack[-1]
                    if top_height <= height:
                        break

                    last_index = top_index
                    height_stack.pop()
                    area = (index - top_index) * top_height
                    max_area = max(area, max_area)

                if top_height == height:
                    continue
                else:
                    height_stack.append((height, last_index))

        return max_area


if __name__ == "__main__":
    problems = [[2, 1, 5, 6, 2, 3], [2, 4], [1, 1], [2, 1, 2]]

    solution = Solution()
    for problem in problems:
        print(solution.largestRectangleArea(problem))
