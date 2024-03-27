#include <charconv>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

std::vector<std::string> parse_input(std::string_view file)
{
    auto data = std::ifstream(file.data());
    auto games = std::vector<std::string>();

    std::string line;
    while (std::getline(data, line))
    {
        auto first_space = line.find_first_of(' ');
        line.erase(line.begin(), line.begin() + first_space + 1);
        games.push_back(line);
    }

    return games;
}

std::pair<int, int> compute(const std::vector<std::string>& games, int max_r, int max_g, int max_b)
{
    int sum = 0;
    int power = 0;

    for (const auto& game : games)
    {
        // Part one
        bool valid = true;

        // Part two
        int min_r = 0;
        int min_g = 0;
        int min_b = 0;

        auto window = std::string_view(game.begin(), game.end());
        size_t begin = 0;
        size_t end = game.size();
        while ((begin = window.find_last_of(":,;", end)) != std::string::npos)
        {
            // game.substr(begin + 2, end) == "x color"
            auto poi = std::string_view(game.begin() + begin + 2, game.begin() + end);
            auto split = poi.find(" ");

            auto color = poi.substr(split + 1);
            int value = 0;
            std::from_chars(poi.begin(), poi.begin() + split, value);

            // Check part one
            if ((color == "red" && value > max_r) || (color == "green" && value > max_g) ||
                (color == "blue" && value > max_b))
            {
                valid = false;
            }

            // Check part two
            if (color == "red" && value > min_r)
            {
                min_r = value;
            }
            else if (color == "green" && value > min_g)
            {
                min_g = value;
            }
            else if (color == "blue" && value > min_b)
            {
                min_b = value;
            }

            end = begin;
            window = std::string_view(game.begin(), game.begin() + end);
        }

        // Add to sum for part one
        if (valid)
        {
            int id = 0;
            std::from_chars(game.data(), game.data() + game.size(), id);
            sum += id;
        }

        // Add to power for part two
        power += min_r * min_g * min_b;
    }

    return std::make_pair(sum, power);
}

std::pair<int, int> compute_ranges(const std::vector<std::string> games, int max_r, int max_g, int max_b)
{
    int sum = 0;
    int power = 0;

    for (const std::string& game : games)
    {
        // for (const auto result_list : std::ranges::split_view(game, ";"))
        // {
        //     std::cout << std::quoted(std::string_view(result_list.begin(), result_list.end())) << '\n';
        //     for (std::cout << "{ "; const auto element : result_list)
        //         std::cout << element << ' ';
        //     std::cout << "} ";
        //     // for (const auto result : std::ranges::views::split(result_list, ","))
        //     // {
        //     //     std::cout << std::quoted(std::string_view(result.begin(), result.end())) << "|";
        //     // }
        //     std::cout << '\n';
        // }

        // auto thung = game | std::views::split(';') | std::views::split(',') |
        //     std::views::transform(
        //         [](auto cube_count) { std::cout << std::string_view(cube_count.begin(), cube_count.end()) << '\n';
        //         });
        auto thung = game | std::views::drop(3) | std::views::split(std::string_view("; ")) |
            // std::views::split(std::string_view(", ")) |
            std::views::transform([](auto v) { return std::string_view(v.begin(), v.end()); });
        for (const auto& t : thung)
        {
            std::cout << t << " | ";
        }

        std::cout << "\n============\n";
    }

    return std::make_pair(sum, power);
}

int main()
{
    auto input = parse_input("in.txt");

    auto [sum, power] = compute(input, 12, 13, 14);
    std::cout << "Sum of IDs of valid games: " << sum << '\n';
    std::cout << "Power of minimal cube sets: " << power << '\n';

    // auto [sum_r, power_r] = compute_ranges(input, 12, 13, 14);
    // std::cout << "Sum of IDs of valid games: " << sum_r << '\n';
    // std::cout << "Power of minimal cube sets: " << power_r << '\n';
}
