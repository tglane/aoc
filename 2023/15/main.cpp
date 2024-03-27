#include <array>
#include <charconv>
#include <fstream>
#include <iostream>
#include <list>
#include <optional>
#include <ranges>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

enum class Operation : uint8_t
{
    Eq = '=',
    Dash = '-',
};

std::vector<std::string> parse_input(std::string_view data)
{
    auto step_range = data | std::views::filter([](char c) { return c != '\n'; }) |
        std::views::split(std::string_view(",")) |
        std::views::transform([](auto r) { return std::string(r.begin(), r.end()); });
    return std::vector(step_range.begin(), step_range.end());
}

int64_t hash(std::string_view input)
{
    int64_t value = 0;

    for (char c : input)
    {
        value += static_cast<int>(c);
        value *= 17;
        value %= 256;
    }

    return value;
}

std::tuple<std::string, Operation, std::optional<int>> parse_sequence_step(std::string_view step)
{
    for (size_t i = 0; i < step.size(); i++)
    {
        if (static_cast<Operation>(step[i]) == Operation::Eq)
        {
            std::optional<int> param = std::nullopt;
            int val = 0;
            auto [p, err] = std::from_chars(step.begin() + i + 1, step.end(), val);
            if (err == std::errc())
            {
                param = val;
            }
            return std::make_tuple(std::string(step.begin(), step.begin() + i), Operation::Eq, param);
        }
        else if (static_cast<Operation>(step[i]) == Operation::Dash)
        {
            return std::make_tuple(std::string(step.begin(), step.begin() + i), Operation::Dash, std::nullopt);
        }
    }
}

size_t HASHMAP(const std::vector<std::string>& init_seq)
{
    auto boxes = std::vector<std::list<std::pair<std::string, int>>>();
    boxes.resize(256);

    for (const auto& step : init_seq)
    {
        const auto [label, op, param] = parse_sequence_step(step);
        int64_t label_hash = hash(label);

        auto& box = boxes[label_hash];
        if (op == Operation::Eq && param.has_value())
        {
            bool modified = false;
            for (auto it = box.begin(); it != box.end(); it++)
            {
                if (it->first == label)
                {
                    it->second = *param;
                    modified = true;
                    break;
                }
            }

            if (!modified)
            {
                box.emplace_back(label, *param);
            }
        }
        else if (op == Operation::Dash)
        {
            for (auto it = box.begin(); it != box.end(); it++)
            {
                if (it->first == label)
                {
                    box.erase(it);
                    break;
                }
            }
        }
    }

    size_t focus_power_sum = 0;
    for (size_t i = 0; i < boxes.size(); i++)
    {
        size_t slot_num = 1;
        for (auto it = boxes[i].begin(); it != boxes[i].end(); it++, slot_num++)
        {
            // power = (box_index + 1) * (slot_index + 1) * focal length
            size_t focusing_power = (i + 1) * slot_num * it->second;
            focus_power_sum += focusing_power;
        }
    }

    return focus_power_sum;
}

int main()
{
    auto input_t = std::string_view("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto init_seq = parse_input(input);

    size_t hash_sum = 0;
    for (const auto& step : init_seq)
    {
        hash_sum += hash(step);
    }
    std::cout << "A) Sum of hashes: " << hash_sum << '\n';

    std::cout << "B) Sum of focusing power: " << HASHMAP(init_seq) << '\n';
}
