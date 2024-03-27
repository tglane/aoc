#include <algorithm>
#include <charconv>
#include <fstream>
#include <iostream>
#include <iterator>
#include <limits>
#include <ranges>
#include <string>
#include <thread>
#include <unordered_map>
#include <vector>

struct translation_t
{
    size_t start;
    size_t end;
    size_t output_offset;
};

struct translation_stage
{
    std::vector<translation_t> translations;

    size_t translate(size_t in) const
    {
        for (const auto& translation : translations)
        {
            if (in >= translation.start && in <= translation.end)
            {
                return in + translation.output_offset;
            }
        }
        return in;
    }
};

struct seed_range
{
    size_t start;
    size_t end;
};

class almanac
{
    std::vector<size_t> m_seeds;
    std::vector<seed_range> m_seed_ranges;

    std::vector<translation_stage> m_stages;

public:
    almanac(std::vector<size_t> seeds, std::vector<translation_stage> mappings)
        : m_seeds(std::move(seeds))
        , m_stages(std::move(mappings))
    {
        for (size_t i = 0; i < m_seeds.size(); i += 2)
        {
            m_seed_ranges.emplace_back(m_seeds[i], m_seeds[i] + m_seeds[i + 1] - 1);
        }
    }

    const std::vector<size_t>& seeds() const
    {
        return m_seeds;
    }

    const std::vector<seed_range>& seed_ranges() const
    {
        return m_seed_ranges;
    }

    const std::vector<translation_stage>& stages() const
    {
        return m_stages;
    }
};

almanac parse_input(std::string_view path)
{
    auto file = std::ifstream(path.data());
    auto file_it = std::istreambuf_iterator<char>(file);

    auto input = std::string(file_it, std::istreambuf_iterator<char>());

    auto chunks = std::views::split(input, std::string_view("\n\n"));

    // Handle the first chunk separately because that contains only the seeds in one single line
    auto seed_chunk = chunks.front();
    auto seed_value_start = std::ranges::search(seed_chunk, std::string_view(":"));
    auto seed_range = std::string_view(seed_value_start.begin() + 2, seed_chunk.end()) | std::views::split(' ') |
        std::views::transform(
            [](auto&& r)
            {
                size_t value = 0;
                std::from_chars(r.begin(), r.end(), value);
                return value;
            });
    auto seeds = std::vector<size_t>(seed_range.begin(), seed_range.end());

    auto stages = std::vector<translation_stage>();
    for (auto&& chunk : std::views::drop(chunks, 1))
    {
        auto value_start = std::ranges::search(chunk, std::string_view(":\n"));
        auto value_view = std::string_view(value_start.begin() + 2, chunk.end());

        auto stage = translation_stage();

        auto mapping_lines = std::views::split(value_view, '\n');
        for (auto&& mapping : std::views::filter(mapping_lines, [](auto&& r) { return !r.empty(); }))
        {
            auto mapping_view = std::string_view(mapping.begin(), mapping.end());

            auto num_range = std::views::split(mapping, ' ');
            auto num_range_it = num_range.begin();

            size_t dest = 0;
            std::from_chars((*num_range_it).begin(), (*num_range_it).end(), dest);
            num_range_it = std::next(num_range_it);

            size_t src = 0;
            std::from_chars((*num_range_it).begin(), (*num_range_it).end(), src);
            num_range_it = std::next(num_range_it);

            size_t dist = 0;
            std::from_chars((*num_range_it).begin(), (*num_range_it).end(), dist);

            stage.translations.emplace_back(src, src + dist - 1, dest - src);
        }

        stages.push_back(stage);
    }

    return almanac(std::move(seeds), std::move(stages));
}

int main()
{
    auto a = parse_input("in.txt");

    // Part 1
    size_t lowest_location = std::numeric_limits<size_t>::max();
    for (auto seed : a.seeds())
    {
        size_t val = seed;
        for (const auto& stage : a.stages())
        {
            val = stage.translate(val);
        }

        lowest_location = std::min(val, lowest_location);
    }

    std::cout << "Lowest location: " << lowest_location << '\n';

    // Part 2 with multiple threads
    auto crunchers = std::vector<std::thread>();
    auto lowest_locations = std::vector<size_t>(a.seed_ranges().size());
    crunchers.reserve(a.seed_ranges().size());

    for (size_t i = 0; i < a.seed_ranges().size(); i++)
    {
        crunchers.push_back(std::thread(
            [i, &lowest_locations, &a]()
            {
                const auto& range = a.seed_ranges()[i];
                std::cout << "Crunch range from " << range.start << " to " << range.end << '\n';

                for (size_t seed = range.start; seed <= range.end; seed++)
                {
                    size_t val = seed;
                    for (const auto& stage : a.stages())
                    {
                        val = stage.translate(val);
                    }

                    lowest_locations[i] = std::min(val, lowest_locations[i]);
                }
            }));
    }

    size_t lowest_location_of_ranges = std::numeric_limits<size_t>::max();
    for (size_t i = 0; i < crunchers.size(); i++)
    {
        crunchers[i].join();
        lowest_location_of_ranges = std::min(lowest_locations[i], lowest_location_of_ranges);
    }
    std::cout << "Lowest location of ranges: " << lowest_location_of_ranges << '\n';
}
