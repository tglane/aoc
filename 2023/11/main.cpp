#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>
#include <string_view>
#include <unordered_set>
#include <vector>

enum class Symbol : uint8_t
{
    Space = '.',
    Galaxy = '#',
};

struct Point
{
    size_t x;
    size_t y;
};

using Grid = std::vector<std::vector<Symbol>>;

Grid parse_input(std::string_view data)
{
    auto parsed = Grid();
    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });
    for (auto&& line : lines)
    {
        auto line_r = line | std::views::transform([](char c) { return static_cast<Symbol>(c); });
        parsed.emplace_back(line_r.begin(), line_r.end());
    }
    return parsed;
}

size_t distance_sum(const Grid& grid, std::size_t spread_factor)
{
    auto empty_rows = std::unordered_set<size_t>();
    for (size_t y = 0; y < grid.size(); y++)
    {
        if (std::all_of(grid[y].begin(), grid[y].end(), [](Symbol s) { return s == Symbol::Space; }))
        {
            empty_rows.insert(y);
        }
    }

    auto galaxies = std::vector<Point>();
    auto empty_cols = std::unordered_set<size_t>();
    for (size_t x = 0; x < grid.front().size(); x++)
    {
        bool all_empty = true;
        for (size_t y = 0; y < grid.size(); y++)
        {
            if (grid[y][x] == Symbol::Galaxy)
            {
                galaxies.emplace_back(x, y);
            }

            if (grid[y][x] != Symbol::Space)
            {
                all_empty = false;
            }
        }

        if (all_empty)
        {
            empty_cols.insert(x);
        }
    }

    size_t dist_sum = 0;
    for (size_t a = 0; a < galaxies.size(); a++)
    {
        for (size_t b = a + 1; b < galaxies.size(); b++)
        {
            // Calculate manhatten dist between galaxies a and b
            const auto& ga = galaxies[a];
            const auto& gb = galaxies[b];
            size_t dist = 0;

            // Rows
            for (size_t row = std::min(ga.y, gb.y); row < std::max(ga.y, gb.y); row++)
            {
                auto it = empty_rows.find(row);
                dist += (it != empty_rows.end()) ? spread_factor : 1;
            }

            // Cols
            for (size_t col = std::min(ga.x, gb.x); col < std::max(ga.x, gb.x); col++)
            {
                auto it = empty_cols.find(col);
                dist += (it != empty_cols.end()) ? spread_factor : 1;
            }

            dist_sum += dist;
        }
    }

    return dist_sum;
}

void test()
{
    auto input = std::string_view(R"(...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....)");

    auto grid = parse_input(input);
    assert(distance_sum(grid, 2) == 374);
    assert(distance_sum(grid, 100) == 8410);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_input(input);
    std::cout << "A) Sum of distances with spread factor 2: " << distance_sum(grid, 2) << '\n';
    std::cout << "A) Sum of distances with spread factor 2: " << distance_sum(grid, 1000000) << '\n';
}
