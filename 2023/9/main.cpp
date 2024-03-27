#include <algorithm>
#include <cassert>
#include <charconv>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string_view>
#include <vector>

std::vector<std::vector<int>> parse_input(std::string_view data)
{
    auto parsed_lines = std::vector<std::vector<int>>();

    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });
    for (auto&& line : lines)
    {
        auto num_range = line | std::views::split(std::string_view(" ")) |
            std::views::transform(
                [](auto&& r)
                {
                    int number = 0;
                    std::from_chars(r.begin(), r.end(), number);
                    return number;
                });
        parsed_lines.emplace_back(num_range.begin(), num_range.end());
    }

    return parsed_lines;
}

size_t predict_value(const std::vector<int>& history, bool forward)
{
    auto diffs = std::vector<int>(history.size() - 1);
    for (size_t i = 0; i < history.size() - 1; i++)
    {
        auto diff = history[i + 1] - history[i];
        diffs[i] = diff;
    }

    if (std::all_of(diffs.begin(), diffs.end(), [](auto num) { return num == 0; }))
    {
        return (forward) ? history.front() : history.back();
    }
    else
    {
        if (forward)
        {
            return history.back() + predict_value(diffs, forward);
        }
        else
        {
            return history.front() - predict_value(diffs, forward);
        }
    }
}

void test()
{
    auto input = std::string_view(R"(0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45)");
    auto history_list = parse_input(input);

    size_t prediction_sum = 0;
    for (const auto& history : history_list)
    {
        prediction_sum += predict_value(history, true);
    }
    std::cout << "[TEST] A) Sum of prediction values: " << prediction_sum << '\n';
    assert(prediction_sum == 114);

    size_t prediction_sum_front = 0;
    for (const auto& history : history_list)
    {
        prediction_sum_front += predict_value(history, false);
    }
    std::cout << "[TEST] B) Sum of front prediction values: " << prediction_sum_front << '\n';
    assert(prediction_sum_front == 2);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input_str = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
    auto history_list = parse_input(input_str);

    size_t prediction_sum = 0;
    size_t prediction_sum_front = 0;
    for (const auto& history : history_list)
    {
        prediction_sum += predict_value(history, true);
        prediction_sum_front += predict_value(history, false);
    }
    std::cout << "A) Sum of prediction values: " << prediction_sum << '\n';
    std::cout << "B) Sum of front prediction values: " << prediction_sum_front << '\n';
}
