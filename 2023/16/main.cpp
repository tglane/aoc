#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <queue>
#include <ranges>
#include <set>
#include <string>
#include <string_view>
#include <vector>

enum class Field : uint8_t
{
    Empty = '.',
    UpMirror = '/',
    DownMirror = '\\',
    HorizontalSplitter = '|',
    VerticalSplitter = '-',
};

enum class Direction : uint8_t
{
    Right = '>',
    Down = 'v',
    Left = '<',
    Up = '^',
};

struct Position
{
    size_t x;
    size_t y;

    Position apply_direction(Direction other_dir) const
    {
        auto new_pos = *this;
        switch (other_dir)
        {
            case Direction::Right:
                new_pos.x++;
                break;
            case Direction::Down:
                new_pos.y++;
                break;
            case Direction::Left:
                new_pos.x--;
                break;
            case Direction::Up:
                new_pos.y--;
                break;
        }
        return new_pos;
    }

    bool operator<(const Position& rhs) const
    {
        if (x == rhs.x)
        {
            return y < rhs.y;
        }
        return x < rhs.x;
    }
};

struct Beam
{
    Position pos;
    Direction dir;
};

class Grid
{
    std::vector<std::vector<Field>> m_data;

    void traverse(Position pos, Direction dir, std::set<std::pair<Position, Direction>>& seen) const
    {

        if (pos.x < 0 || pos.x >= m_data.front().size() || pos.y < 0 || pos.y >= m_data.size())
        {
            // Out of bounds
            return;
        }

        auto key = std::make_pair(pos, dir);
        if (auto it = seen.find(key); it != seen.end())
        {
            // Already visited the position with the direction
            return;
        }
        seen.insert(key);

        switch (m_data[pos.y][pos.x])
        {
            case Field::Empty:
                traverse(pos.apply_direction(dir), dir, seen);
                break;
            case Field::UpMirror:
                switch (dir)
                {
                    case Direction::Right:
                        traverse(pos.apply_direction(Direction::Up), Direction::Up, seen);
                        break;
                    case Direction::Left:
                        traverse(pos.apply_direction(Direction::Down), Direction::Down, seen);
                        break;
                    case Direction::Up:
                        traverse(pos.apply_direction(Direction::Right), Direction::Right, seen);
                        break;
                    case Direction::Down:
                        traverse(pos.apply_direction(Direction::Left), Direction::Left, seen);
                        break;
                }
                break;
            case Field::DownMirror:
                switch (dir)
                {
                    case Direction::Right:
                        traverse(pos.apply_direction(Direction::Down), Direction::Down, seen);
                        break;
                    case Direction::Left:
                        traverse(pos.apply_direction(Direction::Up), Direction::Up, seen);
                        break;
                    case Direction::Up:
                        traverse(pos.apply_direction(Direction::Left), Direction::Left, seen);
                        break;
                    case Direction::Down:
                        traverse(pos.apply_direction(Direction::Right), Direction::Right, seen);
                        break;
                }
                break;
            case Field::HorizontalSplitter:
                switch (dir)
                {
                    case Direction::Right:
                    case Direction::Left:
                        // Split up and down
                        traverse(pos.apply_direction(Direction::Up), Direction::Up, seen);
                        traverse(pos.apply_direction(Direction::Down), Direction::Down, seen);
                        break;
                    case Direction::Up:
                    case Direction::Down:
                        traverse(pos.apply_direction(dir), dir, seen);
                        break;
                }
                break;
            case Field::VerticalSplitter:
                switch (dir)
                {
                    case Direction::Up:
                    case Direction::Down:
                        // Split left and right
                        traverse(pos.apply_direction(Direction::Right), Direction::Right, seen);
                        traverse(pos.apply_direction(Direction::Left), Direction::Left, seen);
                        break;
                    case Direction::Right:
                    case Direction::Left:
                        traverse(pos.apply_direction(dir), dir, seen);
                        break;
                }
                break;
        }
    }

public:
    Grid()
        : m_data{}
    {}

    Grid(std::vector<std::vector<Field>> data)
        : m_data{std::move(data)}
    {}

    size_t max_x() const
    {
        return m_data.front().size() - 1;
    }

    size_t max_y() const
    {
        return m_data.size() - 1;
    }

    template <typename It>
    void emplace_back(It begin, It end)
    {
        m_data.emplace_back(begin, end);
    }

    void print() const
    {
        for (const auto& line : m_data)
        {
            for (Field f : line)
            {
                std::cout << static_cast<char>(f) << ' ';
            }
            std::cout << '\n';
        }
    }

    size_t beam_coverage(Position pos, Direction dir) const
    {
        auto seen = std::set<std::pair<Position, Direction>>();
        traverse(pos, dir, seen);

        auto energized = std::set<Position>();
        for (const auto& pos_pair : seen)
        {
            energized.insert(pos_pair.first);
        }

        return energized.size();
    }
};

Grid parse_input(std::string_view data)
{
    auto grid = Grid();
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    for (auto line : lines)
    {
        auto trans = line | std::views::transform([](char c) { return static_cast<Field>(c); });
        grid.emplace_back(trans.begin(), trans.end());
    }
    return grid;
}

size_t max_energized_coverage(const Grid& grid)
{
    size_t max_coverage = 0;
    for (size_t x = 0; x <= grid.max_x(); x++)
    {
        max_coverage = std::max(grid.beam_coverage(Position{x, 0}, Direction::Down), max_coverage);
        max_coverage = std::max(grid.beam_coverage(Position{x, grid.max_y()}, Direction::Up), max_coverage);
    }
    for (size_t y = 0; y <= grid.max_y(); y++)
    {
        max_coverage = std::max(grid.beam_coverage(Position{0, y}, Direction::Right), max_coverage);
        max_coverage = std::max(grid.beam_coverage(Position{grid.max_x(), y}, Direction::Left), max_coverage);
    }
    return max_coverage;
}

int main()
{
    auto input_t = std::string_view(R"(.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....)");

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_input(input);
    // grid.print();

    std::cout << "A) Energized tiles: " << grid.beam_coverage(Position{0, 0}, Direction::Right) << '\n';
    std::cout << "B) Max energized tiles: " << max_energized_coverage(grid) << '\n';
}
