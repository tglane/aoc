#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ranges>
#include <vector>

class Pattern
{
    std::vector<std::string> m_lines;

public:
    Pattern(std::vector<std::string> lines)
        : m_lines{std::move(lines)}
    {}

    Pattern transpose() const
    {
        auto transposed = std::vector<std::string>();
        transposed.reserve(m_lines.front().size());
        for (size_t i = 0; i < transposed.capacity(); i++)
        {
            transposed.push_back(std::string(' ', m_lines.size()));
        }

        // [i][j] => [j][i]
        //
        // 1 2 3 4
        // 5 6 7 8
        //
        // 1 5
        // 2 6
        // 3 7
        // 4 8
        //
        //
        // 5 1
        // 6 2
        // 7 3
        // 8 4
        //
        for (size_t i = 0; i < m_lines.size(); i++)
        {
            for (size_t j = 0; j < m_lines[i].size(); j++)
            {
                transposed[j][i] = m_lines[i][j];
            }
        }

        return transposed;
    }

    size_t reflection() const
    {
        for (int64_t i = 1; i < m_lines.size(); i++)
        {
            int64_t k = i;
            bool all_mirrored = true;
            for (int64_t j = i - 1; j >= 0 && k < m_lines.size(); j--, k++)
            {
                if (m_lines[j] != m_lines[k])
                {
                    all_mirrored = false;
                    break;
                }
            }

            if (all_mirrored)
            {
                // Mirror found
                return i;
            }
        }

        return 0;
    }

    size_t reflection_with_smudge() const
    {
        for (int64_t i = 1; i < m_lines.size(); i++)
        {
            int64_t k = i;
            size_t non_mirrored = 0;
            for (int64_t j = i - 1; j >= 0 && k < m_lines.size(); j--, k++)
            {
                // if (m_lines[j] != m_lines[k])
                for (size_t x = 0; x < std::min(m_lines[i].size(), m_lines[k].size()); x++)
                {
                    if (m_lines[j][x] != m_lines[k][x])
                        non_mirrored += 1;
                }
            }

            if (non_mirrored == 1)
            {
                // Mirror with one smudge found
                return i;
            }
        }

        return 0;
    }
};

std::vector<Pattern> parse_input(std::string_view data)
{
    auto grids = std::vector<Pattern>();

    auto block_range =
        data | std::views::split(std::string_view("\n\n")) | std::views::filter([](auto&& r) { return !r.empty(); });

    for (auto&& block : block_range)
    {
        auto lines = block | std::views::split(std::string_view("\n")) |
            std::views::transform([](auto&& r) { return std::string(r.begin(), r.end()); }) |
            std::views::filter([](const auto& line) { return !line.empty(); });

        grids.emplace_back(std::vector(lines.begin(), lines.end()));
    }

    return grids;
}

void test()
{
    auto input = std::string_view(R"(
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#)");

    auto blocks = parse_input(input);

    size_t reflection_sum = 0;
    size_t reflection_sum_smudges = 0;
    for (const auto& block : blocks)
    {
        reflection_sum += 100 * block.reflection();
        reflection_sum_smudges += 100 * block.reflection_with_smudge();

        const auto transposed = block.transpose();
        reflection_sum += transposed.reflection();
        reflection_sum_smudges += transposed.reflection_with_smudge();
    }
    assert(reflection_sum == 405);
    assert(reflection_sum_smudges == 400);
}

int main()
{
    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto blocks = parse_input(input);

    size_t reflection_sum = 0;
    size_t reflection_sum_smudges = 0;
    for (const auto& block : blocks)
    {
        reflection_sum += 100 * block.reflection();
        reflection_sum_smudges += 100 * block.reflection_with_smudge();

        const auto transposed = block.transpose();
        reflection_sum += transposed.reflection();
        reflection_sum_smudges += transposed.reflection_with_smudge();
    }
    std::cout << "A) Sum of reflections: " << reflection_sum << '\n';
    std::cout << "B) Sum of reflections with one smudge: " << reflection_sum_smudges << '\n';
}
