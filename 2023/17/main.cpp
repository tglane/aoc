#include <cassert>
#include <charconv>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>
#include <ranges>
#include <set>
#include <string>
#include <string_view>
#include <vector>

using Grid = std::vector<std::vector<int>>;

enum class Direction : int8_t
{
    Up = -1,
    Down = 1,
    Right = 2,
    Left = -2,
    None = 0,
};

bool opposite_dir(Direction a, Direction b)
{
    return std::abs(static_cast<int8_t>(a)) == std::abs(static_cast<int8_t>(b));
}

struct Pos
{
    int64_t x;
    int64_t y;

    bool operator==(const Pos& other) const
    {
        return x == other.x && y == other.y;
    }

    auto operator<=>(const Pos& other) const
    {
        return std::tie(x, y) <=> std::tie(other.x, other.y);
    }

    Pos move(const std::pair<int8_t, int8_t>& delta) const
    {
        auto new_pos = *this;
        new_pos.x += delta.first;
        new_pos.y += delta.second;
        return new_pos;
    }
};

struct Cell
{
    Pos pos;
    size_t cost = 0;

    size_t straight_moves = 0;
    Direction dir;

    auto operator<=>(const Cell& other) const
    {
        return cost <=> other.cost;
    }
};

Grid parse_input(std::string_view data)
{
    auto grid = Grid();
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    grid.reserve(std::distance(lines.begin(), lines.end()));
    for (auto&& line : lines)
    {
        auto nums = line |
            std::views::transform(
                [](char c)
                {
                    int val = 0;
                    std::from_chars(&c, &c + 1, val);
                    return val;
                });
        grid.emplace_back(nums.begin(), nums.end());
    }
    return grid;
}

std::optional<size_t> dijkstra(const Grid& grid,
    const Pos start,
    const Pos dest,
    size_t min_straight_moves = 1,
    size_t max_straight_moves = 3)
{
    auto q = std::priority_queue<Cell, std::vector<Cell>, std::greater<Cell>>();
    auto seen = std::set<std::tuple<Pos, Direction, size_t>>();

    q.push(Cell{start, 0, 0, Direction::None});

    while (!q.empty())
    {
        const auto curr = q.top();
        q.pop();

        if (curr.pos == dest && curr.straight_moves >= min_straight_moves)
            return curr.cost;

        if (seen.contains(std::tie(curr.pos, curr.dir, curr.straight_moves)))
            continue;

        seen.insert(std::make_tuple(curr.pos, curr.dir, curr.straight_moves));

        static const auto direction_actions = std::map<Direction, std::pair<int8_t, int8_t>>{
            {Direction::Up, std::make_pair(0, -1)},
            {Direction::Down, std::make_pair(0, 1)},
            {Direction::Right, std::make_pair(1, 0)},
            {Direction::Left, std::make_pair(-1, 0)},
        };

        if (curr.straight_moves < max_straight_moves && curr.dir != Direction::None)
        {
            const auto new_pos = curr.pos.move(direction_actions.at(curr.dir));
            if (0 <= new_pos.y && new_pos.y < grid.size() && 0 <= new_pos.x && new_pos.x < grid[new_pos.y].size())
            {
                q.push(Cell{.pos = new_pos,
                    .cost = curr.cost + grid[new_pos.y][new_pos.x],
                    .straight_moves = curr.straight_moves + 1,
                    .dir = curr.dir});
            }
        }

        if (curr.straight_moves >= min_straight_moves || curr.dir == Direction::None)
        {
            for (const auto& dir : std::array{
                     Direction::Up,
                     Direction::Down,
                     Direction::Right,
                     Direction::Left,
                 })
            {
                if (dir != curr.dir && !opposite_dir(dir, curr.dir))
                {
                    const auto& delta = direction_actions.at(dir);
                    const auto new_pos = curr.pos.move(delta);
                    if (0 <= new_pos.y && new_pos.y < grid.size() && 0 <= new_pos.x &&
                        new_pos.x < grid[new_pos.y].size())
                    {
                        q.push(Cell{.pos = new_pos,
                            .cost = curr.cost + grid[new_pos.y][new_pos.x],
                            .straight_moves = 1,
                            .dir = dir});
                    }
                }
            }
        }
    }

    return std::nullopt;
}

void test()
{
    const auto input = std::string_view(R"(2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533)");

    auto grid = parse_input(input);

    auto heat_loss_a = dijkstra(
        grid, Pos{0, 0}, Pos{static_cast<int64_t>(grid.front().size() - 1), static_cast<int64_t>(grid.size() - 1)});
    assert(heat_loss_a.value() == 102);

    auto heat_loss_b = dijkstra(grid,
        Pos{0, 0},
        Pos{static_cast<int64_t>(grid.front().size() - 1), static_cast<int64_t>(grid.size() - 1)},
        4,
        10);
    assert(heat_loss_b.value() == 94);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    const auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_input(input);

    auto heat_loss_a = dijkstra(
        grid, Pos{0, 0}, Pos{static_cast<int64_t>(grid.front().size() - 1), static_cast<int64_t>(grid.size() - 1)});
    std::cout << "A) Heat loss with max 3 straight steps: " << heat_loss_a.value() << '\n';

    auto heat_loss_b = dijkstra(grid,
        Pos{0, 0},
        Pos{static_cast<int64_t>(grid.front().size() - 1), static_cast<int64_t>(grid.size() - 1)},
        4,
        10);
    std::cout << "B) Heat loss with min 4 max 10 straight steps: " << heat_loss_b.value() << '\n';
}
