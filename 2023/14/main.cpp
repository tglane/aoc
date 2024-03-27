#include <fstream>
#include <iostream>
#include <map>
#include <ranges>
#include <string_view>
#include <utility>

enum class Field : uint8_t
{
    RoundedRock = 'O',
    CubedRock = '#',
    Empty = '.',
};

class Plattform
{
    std::vector<std::vector<Field>> m_data;

    void rotate_right()
    {
        // Transpose
        for (size_t i = 0; i < m_data.size(); i++)
        {
            for (size_t j = i + 1; j < m_data[i].size(); j++)
            {
                std::swap(m_data[i][j], m_data[j][i]);
            }
        }

        // Reverse individual rows
        for (size_t i = 0; i < m_data.size(); i++)
        {
            size_t low = 0, high = m_data.size() - 1;
            while (low < high)
            {
                std::swap(m_data[i][low], m_data[i][high]);
                low++;
                high--;
            }
        }
    }

public:
    template <typename It>
    void append(It line_begin, It line_end)
    {
        m_data.emplace_back(line_begin, line_end);
    }

    void move_rocks()
    {
        // Move all rocks to the top
        for (int64_t line = 1; line < m_data.size(); line++)
        {
            for (int64_t x = 0; x < m_data[line].size(); x++)
            {
                if (m_data[line][x] == Field::RoundedRock)
                {
                    // Move the rounded rock as far up as possible
                    for (int64_t y = line - 1; y >= 0; y--)
                    {
                        if (m_data[y][x] == Field::RoundedRock || m_data[y][x] == Field::CubedRock)
                        {
                            std::swap(m_data[line][x], m_data[y + 1][x]);
                            break;
                        }
                        else if (y == 0)
                        {
                            std::swap(m_data[line][x], m_data[y][x]);
                            break;
                        }
                    }
                }
            }
        }
    }

    void cycle()
    {
        // A cycle consists of moving rocks to all 4 directions one after each other
        // North, west, south, east
        // Rotating to the right and then moving north is basically the same

        for (uint8_t step = 0; step < 4; step++)
        {
            move_rocks();
            rotate_right();
        }
    }

    void cycle(size_t cnt)
    {
        std::map<std::vector<std::vector<Field>>, size_t> seen_after_cycle;
        std::map<size_t, std::vector<std::vector<Field>>> cached_positions;

        size_t cycle_steps = 0;
        while (cycle_steps++ < cnt)
        {
            cycle();

            if (auto it = seen_after_cycle.find(m_data); it != seen_after_cycle.end())
            {
                // A recurring cycle takes cycle_steps - it->second cycles
                // The recurring cycle began after it->second cycles
                m_data = cached_positions.at((cnt - it->second) % (cycle_steps - it->second) + it->second);

                break;
            }
            else
            {
                seen_after_cycle.insert(std::make_pair(m_data, cycle_steps));
                cached_positions.insert(std::make_pair(cycle_steps, m_data));
            }
        }
    }

    size_t load_factor() const
    {
        size_t total_load = 0;
        for (size_t y = 0; y < m_data.size(); y++)
        {
            for (size_t x = 0; x < m_data[y].size(); x++)
            {
                if (m_data[y][x] == Field::RoundedRock)
                {
                    total_load += m_data.size() - y;
                }
            }
        }
        return total_load;
    }

    void print() const
    {
        for (const auto& line : m_data)
        {
            for (const Field f : line)
            {
                std::cout << (char) f << ' ';
            }
            std::cout << '\n';
        }
    }
};

Plattform parse_input(std::string_view data)
{
    auto plattform = Plattform();

    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    for (auto line : lines)
    {
        auto fields = line | std::views::transform([](char c) { return static_cast<Field>(c); });
        plattform.append(fields.begin(), fields.end());
    }

    return plattform;
}

void a(Plattform grid)
{
    grid.move_rocks();
    std::cout << "A) Load factor: " << grid.load_factor() << '\n';
}

void b(Plattform grid)
{
    grid.cycle(1000000000);
    std::cout << "B) Load factor: " << grid.load_factor() << '\n';
}

int main()
{
    auto input_t = std::string_view(R"(O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....)");

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
    auto grid = parse_input(input);

    a(grid);
    b(std::move(grid));
}
