#include <algorithm>
#include <cassert>
#include <charconv>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <ranges>
#include <span>
#include <string_view>
#include <vector>

enum class Spring : uint8_t
{
    Operational = '.',
    Broken = '#',
    Unknown = '?',
};

class ConditionRecord
{
    std::vector<Spring> m_springs;
    std::vector<size_t> m_block_sizes;

    size_t count_internal(std::span<const Spring> springs,
        std::span<const size_t> blocks,
        std::map<std::pair<size_t, size_t>, size_t>& cache) const
    {
        if (springs.empty())
        {
            return (blocks.empty()) ? 1 : 0;
        }

        if (blocks.empty())
        {
            return (std::any_of(springs.begin(), springs.end(), [](Spring s) { return s == Spring::Broken; })) ? 0 : 1;
        }

        auto state = std::make_pair(springs.size(), blocks.size());
        if (auto it = cache.find(state); it != cache.end())
        {
            return it->second;
        }

        size_t result = 0;

        if (springs.front() == Spring::Operational || springs.front() == Spring::Unknown)
        {
            result += count_internal(springs.last(springs.size() - 1), blocks, cache);
        }

        if (springs.front() == Spring::Broken || springs.front() == Spring::Unknown)
        {
            if (blocks.front() <= springs.size() &&
                !std::any_of(springs.begin(),
                    springs.begin() + blocks.front(),
                    [](Spring s) { return s == Spring::Operational; }) &&
                (blocks.front() == springs.size() || springs[blocks.front()] != Spring::Broken))
            {
                auto sub_span = (blocks.front() == springs.size()) ? std::span<const Spring>()
                                                                   : springs.last(springs.size() - blocks.front() - 1);
                result += count_internal(sub_span, blocks.last(blocks.size() - 1), cache);
            }
        }

        cache.insert(std::make_pair(state, result));

        return result;
    }

public:
    ConditionRecord(std::vector<Spring> springs, std::vector<size_t> blocks)
        : m_springs(std::move(springs))
        , m_block_sizes(std::move(blocks))
    {}

    const std::vector<Spring>& springs() const
    {
        return m_springs;
    }

    size_t count() const
    {
        auto cache = std::map<std::pair<size_t, size_t>, size_t>();
        return count_internal(m_springs, m_block_sizes, cache);
    }

    ConditionRecord unfold(size_t fold_factor) const
    {
        auto unfold_springs = m_springs;
        unfold_springs.reserve(m_springs.size() * 5);
        for (size_t i = 0; i < fold_factor - 1; i++)
        {
            unfold_springs.push_back(Spring::Unknown);
            unfold_springs.insert(unfold_springs.end(), m_springs.begin(), m_springs.end());
        }

        auto unfold_blocks = m_block_sizes;
        unfold_blocks.reserve(m_block_sizes.size() * 4);
        for (size_t i = 0; i < fold_factor - 1; i++)
        {
            unfold_blocks.insert(unfold_blocks.end(), m_block_sizes.begin(), m_block_sizes.end());
        }

        return ConditionRecord(std::move(unfold_springs), std::move(unfold_blocks));
    }
};

std::vector<ConditionRecord> parse_input(std::string_view data)
{
    auto parsed = std::vector<ConditionRecord>();
    auto lines = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });
    for (auto&& line : lines)
    {
        // auto transformed_line = line | std::views::transform([](char c) { return static_cast<Spring>(c); });
        // parsed.emplace_back(transformed_line.begin(), transformed_line.end());
        auto parts = line | std::views::split(std::string_view(" "));
        assert(std::distance(parts.begin(), parts.end()) == 2);

        // Parse springs
        // parts.front();
        auto springs = *parts.begin() | std::views::transform([](char c) { return static_cast<Spring>(c); });

        // Parse block sizes
        auto block_sizes = *std::next(parts.begin()) | std::views::split(std::string_view(",")) |
            std::views::transform(
                [](auto&& r)
                {
                    size_t number = 0;
                    std::from_chars(r.begin(), r.end(), number);
                    return number;
                });

        parsed.emplace_back(std::vector<Spring>(springs.begin(), springs.end()),
            std::vector<size_t>(block_sizes.begin(), block_sizes.end()));
    }
    return parsed;
}

void test()
{
    auto input = std::string_view(R"(???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1)");

    auto records = parse_input(input);
    size_t sum = 0;
    for (const auto& record : records)
    {
        sum += record.count();
    }
    std::cout << "[Test] A) Sum of possibilities: " << sum << '\n';
    assert(sum == 21);

    auto unfolded_sum = 0;
    for (const auto& record : records)
    {
        auto unfolded_record = record.unfold(5);
        unfolded_sum += unfolded_record.count();
    }
    std::cout << "[Test] B) Sum of unfolded possibilities: " << unfolded_sum << '\n';
    assert(unfolded_sum == 525152);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    const auto records = parse_input(input);
    size_t sum = 0;
    for (const auto& record : records)
    {
        sum += record.count();
    }
    std::cout << "A) Sum of possibilies: " << sum << '\n';

    size_t unfolded_sum = 0;
    for (const auto& record : records)
    {
        auto unfolded_record = record.unfold(5);
        unfolded_sum += unfolded_record.count();
    }
    std::cout << "[Test] B) Sum of unfolded possibilities: " << unfolded_sum << '\n';
}
