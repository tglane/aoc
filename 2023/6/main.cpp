#include <cassert>
#include <charconv>
#include <cmath>
#include <fstream>
#include <iostream>
#include <iterator>
#include <ranges>
#include <string>
#include <string_view>
#include <vector>

struct RaceSpecs
{
    double time;
    double dist;

    size_t margin_of_error() const
    {
        // Problem statement:
        // (T - H) * H > D       where T = race time
        //                             H = hold time
        //                             D = record distance
        //
        // Find all H where this expression is true.
        //
        // => TH - H^2 > D
        // => H^2 - TH + D > 0
        //
        // Quadratic formula: aX^2 + bX + C = 0
        // => Two solutions == upper and lower bound of the winning range
        //
        // => a = 1, b = T, c = D

        auto tmp = std::sqrt(std::pow(time, 2) - 4.0 * dist);
        auto first = (-time + tmp) / 2.0;
        auto sec = (-time - tmp) / 2.0;

        if (first == std::floor(first))
        {
            first -= 1.0;
        }

        size_t winning_range = std::abs(std::floor(first) - std::floor(sec));

        return winning_range;
    }

    size_t brute_forced_margin_of_error() const
    {
        size_t winning_range = 0;

        for (size_t speed = 0; speed <= time; speed++)
        {
            size_t range = (time - speed) * speed;

            if (range > dist)
            {
                winning_range += 1;
            }
        }

        return winning_range;
    }
};

std::vector<RaceSpecs> parse_input(std::string_view data)
{
    auto lines = std::views::split(data, '\n');

    auto times_start = std::ranges::search(*lines.begin(), std::string_view(":"));
    auto times = std::string_view(times_start.begin() + 1, lines.front().end()) |
        std::views::split(std::string_view(" ")) | std::views::filter([](auto&& r) { return !r.empty(); }) |
        std::views::transform(
            [](auto&& r)
            {
                int64_t number = 0;
                std::from_chars(r.begin(), r.end(), number);
                return number;
            });
    auto times_vec = std::vector(times.begin(), times.end());

    auto dist_it = std::next(lines.begin(), 1);
    auto dist_start = std::ranges::search(*dist_it, std::string_view(":"));
    auto dists = std::string_view(dist_start.begin() + 1, data.end()) | std::views::split(std::string_view(" ")) |
        std::views::filter([](auto&& r) { return !r.empty(); }) |
        std::views::transform(
            [](auto&& r)
            {
                int64_t number = 0;
                std::from_chars(r.begin(), r.end(), number);
                return number;
            });
    auto dists_vec = std::vector(dists.begin(), dists.end());

    auto races = std::vector<RaceSpecs>();
    for (auto i = 0; i < times_vec.size(); i++)
    {
        races.emplace_back(times_vec[i] * 1.0, dists_vec[i] * 1.0);
    }

    return races;
}

void test()
{
    auto input = std::string_view(R"(Time:      7  15   30
Distance:  9  40  200)");

    auto races = parse_input(input);
    size_t margin_of_errors = 1;
    for (const auto& race : races)
    {
        // margin_of_errors *= race.margin_of_error();
        margin_of_errors *= race.brute_forced_margin_of_error();
    }
    std::cout << "[Test] Margin of errors multiplied: " << margin_of_errors << '\n';
    assert(margin_of_errors == 288);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto file_input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
    auto races = parse_input(file_input);

    size_t margin_of_errors = 1;
    for (const auto& race : races)
    {
        // margin_of_errors *= race.margin_of_error();
        margin_of_errors *= race.brute_forced_margin_of_error();
    }
    std::cout << "Margin of errors multiplied: " << margin_of_errors << '\n';
}
