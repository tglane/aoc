#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <iterator>
#include <limits>
#include <map>
#include <ranges>
#include <set>
#include <string_view>
#include <vector>

enum class Field : uint8_t
{
    Path = '.',
    Forest = '#',
    UpSlope = '^',
    RightSlope = '>',
    DownSlope = 'v',
    LeftSlope = '<',
};

using Point = std::pair<size_t, size_t>;

using Grid = std::vector<std::vector<Field>>;

using AdjacencyList = std::map<Point, std::map<Point, size_t>>;

struct DirectionMap
{
    static const inline std::unordered_map<Field, std::vector<Point>> s_map = {
        std::make_pair(Field::UpSlope, std::vector<Point>{std::make_pair(-1, 0)}),
        std::make_pair(Field::DownSlope, std::vector<Point>{std::make_pair(1, 0)}),
        std::make_pair(Field::LeftSlope, std::vector<Point>{std::make_pair(0, -1)}),
        std::make_pair(Field::RightSlope, std::vector<Point>{std::make_pair(0, 1)}),
        std::make_pair(Field::Path,
            std::vector<Point>{
                std::make_pair(-1, 0),
                std::make_pair(1, 0),
                std::make_pair(0, -1),
                std::make_pair(0, 1),
            }),
    };

    static const std::vector<Point>& at(Field f, bool slippery)
    {
        if (!slippery)
            return s_map.at(Field::Path); // b
        else
            return s_map.at(f); // a
    }
};

Grid parse_grid(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });

    auto grid = Grid();
    for (auto&& line : lines)
    {
        auto fields = line | std::views::transform([](char c) { return static_cast<Field>(c); });
        grid.emplace_back(fields.begin(), fields.end());
    }

    return grid;
}

float dfs(Point end, Point pt, const AdjacencyList& graph, std::set<Point>& seen)
{
    if (pt == end)
    {
        return 0;
    }

    float m = -std::numeric_limits<float>::infinity();

    if (auto pt_it = graph.find(pt); pt_it != graph.end())
    {
        auto [seen_it, flag] = seen.insert(pt);
        for (const auto& nx : pt_it->second)
        {
            if (auto nx_it = seen.find(nx.first); nx_it == seen.end())
            {
                m = std::max(m, dfs(end, nx.first, graph, seen) + pt_it->second.at(nx.first));
            }
        }
        seen.erase(seen_it);
    }

    return m;
}

float longest_scenic_hike(const Grid& grid, bool b)
{
    auto start = std::make_pair(0, 0);
    for (int64_t i = grid.front().size() - 1; i >= 0; i--)
        if (grid.front()[i] == Field::Path)
            start.second = i;
    auto dest = std::make_pair(grid.size() - 1, 0);
    for (int64_t i = grid.back().size() - 1; i >= 0; i--)
        if (grid.back()[i] == Field::Path)
            dest.second = i;

    auto points = std::vector<std::pair<size_t, size_t>>{
        start,
        dest,
    };

    for (size_t row = 0; row < grid.size(); row++)
    {
        for (size_t col = 0; col < grid[row].size(); col++)
        {
            if (grid[row][col] == Field::Forest)
            {
                continue;
            }

            size_t n = 0;
            for (const auto [nr, nc] : std::array<Point, 4>{
                     std::make_pair(row - 1, col),
                     std::make_pair(row + 1, col),
                     std::make_pair(row, col - 1),
                     std::make_pair(row, col + 1),
                 })
            {
                if (0 <= nr && nr < grid.size() && 0 <= nc && nc < grid[nr].size() && grid[nr][nc] != Field::Forest)
                {
                    n++;
                }
            }

            if (n >= 3)
                points.emplace_back(row, col);
        }
    }

    // Create adjacency list
    auto graph = std::map<Point, std::map<Point, size_t>>();
    for (const auto& p : points)
    {
        graph.insert_or_assign(p, std::map<std::pair<size_t, size_t>, size_t>());
    }

    for (const auto& pt : points)
    {
        auto stack = std::vector{std::make_pair(pt, 0)};
        auto seen = std::set{pt};

        while (!stack.empty())
        {
            auto [p, n] = stack.back();
            stack.pop_back();

            auto it = std::find(points.begin(), points.end(), p);
            if (n != 0 && it != points.end())
            {
                graph.at(pt).insert_or_assign(p, n);
                continue;
            }

            for (const auto& d : DirectionMap::at(grid[p.first][p.second], !b))
            {
                size_t nr = p.first + d.first;
                size_t nc = p.second + d.second;
                auto n_it = seen.find(std::make_pair(nr, nc));
                if (0 <= nr && nr < grid.size() && 0 <= nc && nc < grid[nr].size() && grid[nr][nc] != Field::Forest &&
                    n_it == seen.end())
                {
                    auto n_p = std::make_pair(nr, nc);
                    stack.push_back(std::make_pair(n_p, n + 1));
                    seen.insert(n_p);
                }
            }
        }
    }

    // Print adjacency list
    // for (const auto& t : graph)
    // {
    //     std::cout << "(" << t.first.first << ',' << t.first.second << "):\n";
    //     for (const auto& i : t.second)
    //     {
    //         std::cout << "    (" << i.first.first << ',' << i.first.second << ") => " << i.second << "\n";
    //     }
    // }

    // Brute-force longest path
    auto seen = std::set<std::pair<size_t, size_t>>();
    return dfs(dest, start, graph, seen);
}

void test()
{
    auto input = std::string_view(R"(#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#)");

    auto grid = parse_grid(input);
    assert(longest_scenic_hike(grid, false) == 94);
    assert(longest_scenic_hike(grid, true) == 154);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_grid(input);
    std::cout << "A) Steps in longest scenic hike: " << longest_scenic_hike(grid, false) << '\n';
    std::cout << "B) Steps in longest scenic hike without slippery slopes: " << longest_scenic_hike(grid, true) << '\n';
}
