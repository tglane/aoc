#include <cassert>
#include <charconv>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

enum class Direction : uint8_t
{
    Up = 'U',
    Right = 'R',
    Down = 'D',
    Left = 'L',
};

struct Instruction
{
    Direction dir;
    int64_t dist;
    std::string color;

    Instruction decode_color() const
    {
        int64_t new_dist = 0;
        std::from_chars(color.data(), color.data() + color.size() - 1, new_dist, 16);

        // 0 means R, 1 means D, 2 means L, and 3 means U
        Direction new_dir = dir;
        switch (color.back())
        {
            case '0':
                new_dir = Direction::Right;
                break;
            case '1':
                new_dir = Direction::Down;
                break;
            case '2':
                new_dir = Direction::Left;
                break;
            case '3':
                new_dir = Direction::Up;
                break;
        }

        return Instruction{new_dir, new_dist, color};
    }
};

std::vector<Instruction> parse_input(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); }) |
        std::views::transform(
            [](auto line)
            {
                auto parts = line | std::views::split(' ');
                auto parts_it = parts.begin();

                auto dir = static_cast<Direction>((*parts_it).front());
                std::advance(parts_it, 1);

                int64_t dist = 0;
                std::from_chars((*parts_it).begin(), (*parts_it).end(), dist);
                std::advance(parts_it, 1);

                auto color = std::string((*parts_it).begin() + 2, (*parts_it).end() - 1);

                return Instruction{dir, dist, std::move(color)};
            });
    return std::vector<Instruction>(lines.begin(), lines.end());
}

size_t part_one(const std::vector<Instruction>& instructions, bool decode_color)
{
    const auto dir_map = std::unordered_map<Direction, std::pair<int64_t, int64_t>>{
        {Direction::Up, std::make_pair(-1, 0)},
        {Direction::Right, std::make_pair(0, 1)},
        {Direction::Down, std::make_pair(1, 0)},
        {Direction::Left, std::make_pair(0, -1)},
    };

    auto trench = std::vector<std::pair<int64_t, int64_t>>{std::make_pair(0, 0)};
    auto boundary_points = 0;

    for (const auto& inst : instructions)
    {
        const auto& last = trench.back();
        if (decode_color)
        {
            auto decoded_inst = inst.decode_color();

            const auto [dx, dy] = dir_map.at(decoded_inst.dir);

            trench.emplace_back(last.first + (dx * decoded_inst.dist), last.second + (dy * decoded_inst.dist));
            boundary_points += decoded_inst.dist;
        }
        else
        {
            const auto [dx, dy] = dir_map.at(inst.dir);

            trench.emplace_back(last.first + (dx * inst.dist), last.second + (dy * inst.dist));
            boundary_points += inst.dist;
        }
    }

    int64_t inner_area = 0;
    for (int64_t i = 0; i < trench.size(); i++)
    {
        inner_area +=
            trench[i].first * (trench[(i - 1) % trench.size()].second - trench[(i + 1) % trench.size()].second);
    }
    inner_area = std::abs(inner_area) / 2;

    auto i = inner_area - boundary_points / 2 + 1;

    return i + boundary_points;
}

void test()
{
    const auto input = std::string_view(R"(R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3))");
    const auto dig_plan = parse_input(input);

    size_t cubic_lava = part_one(dig_plan, false);
    assert(cubic_lava == 62);

    size_t cubic_lava_from_color = part_one(dig_plan, true);
    assert(cubic_lava_from_color == 952408144115);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    const auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto dig_plan = parse_input(input);
    size_t cubic_lava = part_one(dig_plan, false);
    std::cout << "A) Cubic meters of lava: " << cubic_lava << '\n';
    size_t cubic_lava_from_color = part_one(dig_plan, true);
    std::cout << "B) Cubic meters of lava from decoded color: " << cubic_lava_from_color << '\n';
}
