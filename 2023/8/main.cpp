#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string_view>
#include <unordered_map>
#include <vector>

using NodesMap = std::unordered_map<std::string, std::pair<std::string, std::string>>;

enum class Direction : char
{
    LEFT = 'L',
    RIGHT = 'R',
};

class Network
{
    std::vector<Direction> m_instructions;
    NodesMap m_nodes;

public:
    Network(std::vector<Direction> instructions, NodesMap nodes)
        : m_instructions{std::move(instructions)}
        , m_nodes{std::move(nodes)}
    {}

    std::optional<size_t> start_to_end() const
    {
        auto curr = m_nodes.find("AAA");

        for (size_t i = 0;; i++)
        {
            if (curr == m_nodes.end())
            {
                return std::nullopt;
            }

            if (curr->first == "ZZZ")
            {
                return i;
            }

            auto instruction = m_instructions[i % m_instructions.size()];
            if (instruction == Direction::LEFT)
                curr = m_nodes.find(curr->second.first);
            else
                curr = m_nodes.find(curr->second.second);
        }
    }

    std::vector<std::optional<size_t>> start_to_end_multiple() const
    {
        auto starts = m_nodes | std::views::filter([](const auto& entry) { return entry.first.back() == 'A'; }) |
            std::views::transform([](const auto& entry) { return entry.first; }) | std::ranges::to<std::vector>();

        auto results = std::vector<std::optional<size_t>>();
        results.reserve(starts.size());
        for (const auto& start : starts)
        {
            auto curr = m_nodes.find(start);

            for (size_t i = 0;; i++)
            {
                if (curr == m_nodes.end())
                {
                    results.push_back(std::nullopt);
                    break;
                }

                if (curr->first.back() == 'Z')
                {
                    results.push_back(i);
                    break;
                }

                auto instruction = m_instructions[i % m_instructions.size()];
                if (instruction == Direction::LEFT)
                    curr = m_nodes.find(curr->second.first);
                else
                    curr = m_nodes.find(curr->second.second);
            }
        }

        return results;
    }
};

template <typename Iterator>
size_t multi_lcm(Iterator begin, Iterator end)
{
    size_t multi_lcm = *begin;
    for (auto it = std::next(begin, 1); it != end; it++)
    {
        multi_lcm = std::lcm(*it, multi_lcm);
    }
    return multi_lcm;
}

Network parse_input(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });

    auto instructions = lines.front() | std::views::transform([](char c) { return static_cast<Direction>(c); }) |
        std::ranges::to<std::vector>();

    auto nodes = NodesMap();
    for (auto&& line : lines | std::views::drop(1))
    {
        auto line_parts = line | std::views::split(std::string_view(" = "));
        auto id = std::string(line_parts.front().begin(), line_parts.front().end());

        auto map_part_iter = std::next(line_parts.begin(), 1);
        auto map_parts = std::ranges::subrange((*map_part_iter).begin() + 1, (*map_part_iter).end() - 1);

        auto targets_range = map_parts | std::views::split(std::string_view(", ")) |
            std::views::transform([](auto&& r) { return std::string(r.begin(), r.end()); });
        auto targets_pair = std::make_pair(*targets_range.begin(), *(std::next(targets_range.begin(), 1)));

        nodes.insert(std::make_pair(std::move(id), std::move(targets_pair)));
    }

    return Network(std::move(instructions), std::move(nodes));
}

void test()
{
    auto input_single = std::string_view(R"(LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ))");

    auto network_single = parse_input(input_single);
    auto steps_single = network_single.start_to_end();
    assert(steps_single.value() == 6);

    auto input_multi = std::string_view(R"(LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX))");

    auto network_multi = parse_input(input_multi);
    auto step_results = network_multi.start_to_end_multiple();
    auto steps_list = step_results |
        std::views::filter([](const std::optional<size_t>& opt_res) { return opt_res.has_value(); }) |
        std::views::transform([](const std::optional<size_t>& opt_res) { return opt_res.value(); });
    auto steps_multi = multi_lcm(steps_list.begin(), steps_list.end());
    assert(steps_multi == 6);
    for (const auto& num : steps_list)
        std::cout << "Test: " << num << '\n';
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto network = parse_input(input);

    auto steps = network.start_to_end();
    if (steps)
    {
        std::cout << "A) Start to end: " << steps.value() << '\n';
    }
    else
    {
        std::cout << "A) Start to end: No path found\n";
    }

    auto step_results = network.start_to_end_multiple();
    auto step_list = step_results |
        std::views::filter([](const std::optional<size_t>& opt_res) { return opt_res.has_value(); }) |
        std::views::transform([](const std::optional<size_t>& opt_res) { return opt_res.value(); });
    auto steps_multi = multi_lcm(step_list.begin(), step_list.end());
    std::cout << "B) Start to end: " << steps_multi << '\n';
}
