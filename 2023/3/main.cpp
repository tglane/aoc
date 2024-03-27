#include <algorithm>
#include <cassert>
#include <charconv>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <optional>
#include <ranges>
#include <set>
#include <string_view>
#include <utility>
#include <vector>

std::vector<std::string> parse_input(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); }) |
        std::views::transform([](auto&& line) { return std::string(line.begin(), line.end()); });
    return std::vector<std::string>(lines.begin(), lines.end());
}

bool char_is_digit(char c)
{
    return (c >= 48 && c <= 57);
}

std::optional<char> is_symbol_adjacent(const std::vector<std::string>& schematic, int64_t y, int64_t x)
{
    int64_t min_x = (x - 1 < 0) ? 0 : x - 1;
    int64_t max_x = (x + 1 >= schematic[y].size()) ? schematic[y].size() - 1 : x + 1;
    int64_t min_y = (y - 1 < 0) ? 0 : y - 1;
    int64_t max_y = (y + 1 >= schematic.size()) ? schematic.size() - 1 : y + 1;

    for (size_t i = min_y; i <= max_y; i++)
    {
        for (size_t j = min_x; j <= max_x; j++)
        {
            if (schematic[i][j] != '.' && !char_is_digit(schematic[i][j]))
                return schematic[i][j];
        }
    }

    return std::nullopt;
}

std::optional<std::pair<size_t, size_t>>
adjacent_gear_pos(const std::vector<std::string>& schematic, int64_t y, int64_t x)
{
    int64_t min_x = (x - 1 < 0) ? 0 : x - 1;
    int64_t max_x = (x + 1 >= schematic[y].size()) ? schematic[y].size() - 1 : x + 1;
    int64_t min_y = (y - 1 < 0) ? 0 : y - 1;
    int64_t max_y = (y + 1 >= schematic.size()) ? schematic.size() - 1 : y + 1;

    for (size_t i = min_y; i <= max_y; i++)
    {
        for (size_t j = min_x; j <= max_x; j++)
        {
            if (schematic[i][j] == '*')
                return std::make_pair(i, j);
        }
    }

    return std::nullopt;
}

std::pair<size_t, size_t> calculate(const std::vector<std::string> schematic)
{
    size_t part_number_sum = 0;
    size_t gear_ratio_sum = 0;
    auto gears_ratio = std::map<std::pair<size_t, size_t>, std::pair<size_t, size_t>>();

    for (size_t i = 0; i < schematic.size(); i++)
    {
        std::optional<size_t> number_start = std::nullopt;
        std::optional<size_t> number_end = std::nullopt;
        bool is_part_number = false;

        auto adjacent_gears = std::set<std::pair<size_t, size_t>>();

        for (size_t j = 0; j < schematic[i].size(); j++)
        {

            // Check if we are at a digit
            // If so, check any symbol adjacent to schematic[i][j] is a symbol (-> != '.')
            bool is_digit = char_is_digit(schematic[i][j]);
            if (is_digit && !is_part_number)
            {
                // Check if there is a symbol adjacent to the char
                // is_part_number = is_symbol_adjacent(schematic, i, j);
                if (auto adjacent_char = is_symbol_adjacent(schematic, i, j); adjacent_char)
                {
                    is_part_number = true;
                }

                if (auto gear_pos = adjacent_gear_pos(schematic, i, j); gear_pos)
                {
                    adjacent_gears.insert(*gear_pos);
                }
            }
            if (is_digit)
            {
                // If it is just a digit we set the boundaries of the complete number
                if (!number_start)
                    number_start = j;
                number_end = j;
            }

            if ((!is_digit || j == schematic[i].size() - 1) && is_part_number && number_start && number_end)
            {
                // Construct part_number
                int part_number = 0;
                std::from_chars(
                    schematic[i].data() + *number_start, schematic[i].data() + *number_end + 1, part_number);
                // std::cout << "Part number is " << part_number << '\n';

                number_start = std::nullopt;
                number_end = std::nullopt;
                is_part_number = false;

                part_number_sum += part_number;

                for (const auto& gear : adjacent_gears)
                {
                    if (gears_ratio.contains(gear))
                    {
                        auto& val = gears_ratio.at(gear);
                        val.first += 1;
                        val.second *= part_number;
                    }
                    else
                    {
                        gears_ratio.insert(std::make_pair(gear, std::make_pair(1, part_number)));
                    }
                }
                adjacent_gears.clear();
            }
            else if (!is_digit)
            {
                number_start = std::nullopt;
                number_end = std::nullopt;
                is_part_number = false;

                adjacent_gears.clear();
            }
        }
    }

    // Calc gear ratio sum
    for (const auto& gear_ratio : gears_ratio)
    {
        if (gear_ratio.second.first == 2)
        {
            gear_ratio_sum += gear_ratio.second.second;
        }
    }

    // Apparently std::ranges::fold_right is currently not implemented in clang17
    // auto tmp = std::ranges::fold_right(
    //     gears_ratio | std::views::filter([](const auto& gear_ratio) { return gear_ratio.second.first == 2; }),
    //     0,
    //     std::plus<>());

    return std::make_pair(part_number_sum, gear_ratio_sum);
}

void test()
{
    auto input = std::string_view(R"(467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..)");

    auto grid = parse_input(input);
    auto [part_sum_number, gear_ratio_sum] = calculate(grid);
    assert(part_sum_number == 4361);
    assert(gear_ratio_sum == 467835);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
    auto grid = parse_input(input);

    auto [part_number_sum, gear_ratio_sum] = calculate(grid);
    std::cout << "Sum of partnumbers: " << part_number_sum << '\n';
    std::cout << "Sum of gear ratios: " << gear_ratio_sum << '\n';
}
