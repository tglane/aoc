#include <cassert>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <set>
#include <string_view>
#include <utility>
#include <vector>

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the
// pipe has.

struct Point
{
    size_t x;
    size_t y;
};

class Pipe
{
    enum class Direction
    {
        Up,
        Right,
        Down,
        Left,
    };

    enum class Symbol : uint8_t
    {
        Vertical = '|',
        Horizontal = '-',
        NorthEast = 'L',
        NorthWest = 'J',
        SouthWest = '7',
        SouthEast = 'F',
        Start = 'S',
        Ground = '.',
    };

    bool next_valid_dir(Direction dir, Symbol sym) const
    {
        switch (dir)
        {
            case Direction::Up:
                // Up: F, 7, |
                return sym == Symbol::SouthEast || sym == Symbol::SouthWest || sym == Symbol::Vertical ||
                    sym == Symbol::Start;
            case Direction::Down:
                // Down: L, J, |
                return sym == Symbol::NorthWest || sym == Symbol::NorthEast || sym == Symbol::Vertical ||
                    sym == Symbol::Start;
            case Direction::Right:
                // Right: -, 7, J
                return sym == Symbol::SouthWest || sym == Symbol::NorthWest || sym == Symbol::Horizontal ||
                    sym == Symbol::Start;
            case Direction::Left:
                // Left: -, F, L
                return sym == Symbol::SouthEast || sym == Symbol::NorthEast || sym == Symbol::Horizontal ||
                    sym == Symbol::Start;
        }
    }

public:
    using Symbol = Symbol;

    Pipe(size_t x, size_t y, char c)
        : m_value{static_cast<Symbol>(c)}
        , m_pos{x, y}
    {}

    size_t get_x() const
    {
        return m_pos.x;
    }

    size_t get_y() const
    {
        return m_pos.y;
    }

    Symbol get_symbol() const
    {
        return m_value;
    }

    void set_symbol(Symbol s)
    {
        m_value = s;
    }

    bool is_start() const
    {
        return m_value == Symbol::Start;
    }

    bool is_valid_neighbour(const Pipe& other) const
    {
        auto dir = std::optional<Direction>(std::nullopt);
        if (m_pos.y >= 1 && other.get_x() == m_pos.x && other.get_y() == m_pos.y - 1)
        {
            // Up
            dir = Direction::Up;
        }
        else if (other.get_y() == m_pos.y && other.get_x() == m_pos.x + 1)
        {
            // Right
            dir = Direction::Right;
        }
        else if (other.get_x() == m_pos.x && other.get_y() == m_pos.y + 1)
        {
            // Down
            dir = Direction::Down;
        }
        else if (other.get_y() == m_pos.y && other.get_x() == m_pos.x - 1)
        {
            // Left
            dir = Direction::Left;
        }

        switch (m_value)
        {
            case Symbol::Vertical: // |
                // Down: L, J, |
                // Up: F, 7, |
                if (*dir != Direction::Down && *dir != Direction::Up)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::Horizontal: // -
                // Right: -, 7, J
                // Left: -, F, L
                if (*dir != Direction::Right && *dir != Direction::Left)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::NorthEast: // L
                // Up: |, 7, F
                // Right: -, 7, J
                if (*dir != Direction::Up && *dir != Direction::Right)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::NorthWest: // J
                // Up, |, 7, F
                // Left: -, F, L
                if (*dir != Direction::Up && *dir != Direction::Left)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::SouthWest: // 7
                // Down
                // Left
                if (*dir != Direction::Down && *dir != Direction::Left)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::SouthEast: // F
                // Down
                // Right
                if (*dir != Direction::Down && *dir != Direction::Right)
                    return false;
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::Start:
                // Up
                // Down
                // Right
                // Left
                return next_valid_dir(*dir, other.get_symbol());
            case Symbol::Ground:
                // Cannot happen, is not a pipe
                return false;
        }

        // Point is not at a position where connecting pipes are possible
        return false;
    }

    bool operator==(const Pipe& rhs) const
    {
        return rhs.get_symbol() == m_value && rhs.get_x() == m_pos.x && rhs.get_y() == m_pos.y;
    }

    bool operator!=(const Pipe& rhs) const
    {
        return !(*this == rhs);
    }

private:
    Symbol m_value;
    Point m_pos;
};

using Grid = std::vector<std::vector<Pipe>>;

class Path
{
    std::vector<Pipe> m_inner;
    std::set<std::pair<size_t, size_t>> m_cache;

    bool equal_to_last(const Pipe& potential_new) const
    {
        if (m_inner.size() >= 2)
        {
            return m_inner[m_inner.size() - 2] == potential_new;
        }
        else
        {
            return false;
        }
    }

public:
    Path(Pipe start)
        : m_inner{start}
        , m_cache{{start.get_x(), start.get_y()}}
    {}

    bool contains(const Pipe& other) const
    {
        if (auto it = m_cache.find({other.get_x(), other.get_y()}); it != m_cache.end())
        {
            return true;
        }
        return false;
    }

    bool next(const Grid& grid)
    {
        const auto& curr = m_inner.back();

        // Find a valid pipe connection as neighbour in the grid
        if (curr.get_y() >= 1)
        {
            // Up
            const auto& potential_next = grid[curr.get_y() - 1][curr.get_x()];
            if (!equal_to_last(potential_next) && curr.is_valid_neighbour(potential_next))
            {
                m_inner.push_back(potential_next);
                m_cache.insert({potential_next.get_x(), potential_next.get_y()});
                return true;
            }
        }
        if (curr.get_x() + 1 < grid[curr.get_y()].size())
        {
            // Right
            const auto& potential_next = grid[curr.get_y()][curr.get_x() + 1];
            if (!equal_to_last(potential_next) && curr.is_valid_neighbour(potential_next))
            {
                m_inner.push_back(potential_next);
                m_cache.insert({potential_next.get_x(), potential_next.get_y()});
                return true;
            }
        }
        if (curr.get_y() + 1 < grid.size())
        {
            // Down
            const auto& potential_next = grid[curr.get_y() + 1][curr.get_x()];
            if (!equal_to_last(potential_next) && curr.is_valid_neighbour(potential_next))
            {
                m_inner.push_back(potential_next);
                m_cache.insert({potential_next.get_x(), potential_next.get_y()});
                return true;
            }
        }
        if (curr.get_x() >= 1)
        {
            // Left
            const auto& potential_next = grid[curr.get_y()][curr.get_x() - 1];
            if (!equal_to_last(potential_next) && curr.is_valid_neighbour(potential_next))
            {
                m_inner.push_back(potential_next);
                m_cache.insert({potential_next.get_x(), potential_next.get_y()});
                return true;
            }
        }

        // No move could be made
        return false;
    }

    bool is_loop() const
    {
        return m_inner.back() == m_inner.front();
    }

    size_t size() const
    {
        return m_inner.size();
    }

    friend std::ostream& operator<<(std::ostream& stream, const Path& path)
    {
        for (const Pipe& p : path.m_inner)
        {
            std::cout << " (" << static_cast<char>(p.get_symbol()) << ") ->";
        }

        return stream;
    }
};

Grid parse_input(std::string_view data)
{
    auto grid = Grid();

    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });
    size_t y = 0;
    for (auto&& line : lines)
    {
        size_t x = 0;
        auto symbols = line | std::views::transform([&x, y](char c) { return Pipe(x++, y, c); });

        grid.emplace_back(symbols.begin(), symbols.end());
        y++;
    }

    return grid;
}

std::optional<Pipe> detect_start(const Grid& grid)
{
    for (size_t y = 0; y < grid.size(); y++)
    {
        for (size_t x = 0; x < grid[y].size(); x++)
        {
            if (grid[y][x].is_start())
            {
                return grid[y][x];
            }
        }
    }
    return std::nullopt;
}

Path detect_loop(const Grid& grid)
{
    auto start = detect_start(grid).value();
    auto path = Path(std::move(start));

    do
    {
        if (!path.next(grid))
            break;

    } while (!path.is_loop());

    return path;
}

size_t detect_farthest_away(const Grid& grid)
{
    // To calculate the distance to the point that is the farthest away from the start point we first scan the loop and
    // then the point the farthest away is just the half of the loop length
    auto loop = detect_loop(grid);
    return loop.size() / 2;
}

size_t enclosed_area(Grid& grid, Pipe::Symbol start_replacement)
{
    // To calculate the enclosed area we scan every line for start and end symbols of the loop and count every tile
    // that is between those start and end markers as the area inside

    size_t area = 0;

    auto loop = detect_loop(grid);
    auto start = detect_start(grid).value();

    grid[start.get_y()][start.get_x()].set_symbol(start_replacement);

    for (size_t y = 0; y < grid.size(); y++)
    {
        bool inside = false;
        for (size_t x = 0; x < grid[y].size(); x++)
        {
            if (loop.contains(grid[y][x]))
            {
                const auto sym = grid[y][x].get_symbol();
                if (sym == Pipe::Symbol::Vertical || sym == Pipe::Symbol::NorthEast || sym == Pipe::Symbol::NorthWest)
                {
                    inside = !inside;
                }
            }
            else
            {
                area += (inside) ? 1 : 0;
            }
        }
    }

    return area;
}

void test()
{
    auto input_t = std::string_view(R"(.....
.S-7.
.|.|.
.L-J.
.....)");
    auto grid_t = parse_input(input_t);
    assert(detect_farthest_away(grid_t) == 4);
    assert(enclosed_area(grid_t, Pipe::Symbol::SouthWest) == 1);

    auto input_tt = std::string_view(R"(..F7.
.FJ|.
SJ.L7
|F--J
LJ...)");
    auto grid_tt = parse_input(input_tt);
    assert(detect_farthest_away(grid_tt) == 8);

    auto input_ttt = std::string_view(R"(..........
.F-------7.
.|F-----7|.
.||.....||.
.||.....||.
.SL-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........)");
    auto grid_ttt = parse_input(input_ttt);
    assert(enclosed_area(grid_ttt, Pipe::Symbol::Vertical) == 4);

    auto input_tttt = std::string_view(R"(.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...)");
    auto grid_tttt = parse_input(input_tttt);
    assert(enclosed_area(grid_tttt, Pipe::Symbol::SouthWest) == 8);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto grid = parse_input(input);

    std::cout << "A) Steps to point farthest away: " << detect_farthest_away(grid) << '\n';
    std::cout << "B) Area enclosed: " << enclosed_area(grid, Pipe::Symbol::Vertical) << '\n';
}
