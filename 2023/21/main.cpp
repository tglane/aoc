#include <fstream>
#include <iostream>
#include <ranges>
#include <set>
#include <string_view>
#include <vector>

enum class Field : uint8_t
{
    Start = 'S',
    GardenPlot = '.',
    Rock = '#',
};

using Grid = std::vector<std::vector<Field>>;

Grid parse_input(std::string_view data)
{
    auto grid = Grid();
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    for (auto&& line : lines)
    {
        auto parsed = line | std::views::transform([](char c) { return static_cast<Field>(c); });
        grid.emplace_back(parsed.begin(), parsed.end());
    }
    return grid;
}

size_t part_one(const Grid& grid, size_t steps)
{
    auto positions = std::set<std::pair<size_t, size_t>>();

    for (size_t y = 0; y < grid.size(); y++)
    {
        for (size_t x = 0; x < grid[y].size(); x++)
        {
            if (grid[y][x] == Field::Start)
            {
                positions.insert(std::make_pair(x, y));
            }
        }
    }

    while (steps-- > 0)
    {
        auto next_positions = std::set<std::pair<size_t, size_t>>();
        for (const auto& pos : positions)
        {
            // Create all new positions and insert them into next_positions
            // Up
            if (pos.second >= 1 && grid[pos.second - 1][pos.first] != Field::Rock)
                next_positions.insert(std::make_pair(pos.first, pos.second - 1));
            // Right
            if (pos.first < grid[pos.second].size() - 1 && grid[pos.second][pos.first + 1] != Field::Rock)
                next_positions.insert(std::make_pair(pos.first + 1, pos.second));
            // Down
            if (pos.second < grid.size() - 1 && grid[pos.second + 1][pos.first] != Field::Rock)
                next_positions.insert(std::make_pair(pos.first, pos.second + 1));
            // Left
            if (pos.first >= 1 && grid[pos.second][pos.first - 1] != Field::Rock)
                next_positions.insert(std::make_pair(pos.first - 1, pos.second));
        }

        positions = next_positions;
    }

    return positions.size();
}

int main()
{
    auto input_t = std::string_view(R"(...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........)");

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_input(input);
    size_t steps = 64;
    size_t positions = part_one(grid, steps);
    std::cout << "A) Possitble positions after " << steps << " steps: " << part_one(grid, steps) << '\n';
}
