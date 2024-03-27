#include <algorithm>
#include <cstring>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

std::vector<std::string> parse_input(std::string_view path)
{
    auto file = std::ifstream(path.data());
    auto file_it = std::istream_iterator<std::string>(file);
    auto lines = std::vector<std::string>();
    std::copy(file_it, std::istream_iterator<std::string>(), std::back_inserter(lines));
    return lines;
}

void replace_words_with_nums(std::string& line)
{
    auto words = std::unordered_map<const char*, const char*>{{"one", "1"},
        {"two", "2"},
        {"three", "3"},
        {"four", "4"},
        {"five", "5"},
        {"six", "6"},
        {"seven", "7"},
        {"eight", "8"},
        {"nine", "9"}};

    for (const auto& word : words)
    {
        for (size_t i = 0; i < line.size(); i++)
        {
            auto remaining = std::string_view(line.begin() + i, line.end());
            if (remaining.starts_with(word.first))
            {
                line.insert(i, word.second);
                line.insert(i, 1, line[i + 1]);
                i += 2;
            }
        }
    }
}

int calibration_value(std::vector<std::string>& lines)
{
    auto sum = 0;

    for (const auto& line : lines)
    {
        auto first = 0;
        for (int i = 0; i < line.size(); i++)
        {
            // Char range [48 - 57]
            if (line[i] >= 48 && line[i] <= 57)
            {
                first = line[i] - 48;
                break;
            }
        }

        auto sec = 0;
        for (int i = line.size() - 1; i >= 0; i--)
        {
            // Char range [48 - 57]
            if (line[i] >= 48 && line[i] <= 57)
            {
                sec = line[i] - 48;
                break;
            }
        }

        sum += (first * 10 + sec);
    }

    return sum;
}

void test()
{
    constexpr auto str = std::string_view{R"(two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen)"};

    auto it = std::istream_iterator<std::string>();

    auto out = std::vector<std::string>();
    std::copy(it, std::istream_iterator<std::string>(), std::back_inserter(out));

    for (const auto& line : out)
    {
        std::cout << "Line: " << line << '\n';
    }
}

int main()
{
    test();
    return 0;

    auto lines = parse_input("in.txt");

    auto cal_val = calibration_value(lines);
    std::cout << "Calibration value: " << cal_val << '\n';

    for (auto& line : lines)
    {
        replace_words_with_nums(line);
    }
    auto mod_cal_val = calibration_value(lines);
    std::cout << "Modified calibration value: " << mod_cal_val << '\n';
}
