#include <charconv>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <ranges>
#include <set>
#include <string_view>
#include <vector>

struct game
{
    std::set<int> winning_nums;
    std::vector<int> nums;

    size_t points() const
    {
        size_t winning_count = wins();
        return (winning_count > 0) ? std::pow(2, winning_count - 1) : 0;
    }

    size_t wins() const
    {
        size_t winning_count = 0;
        for (int num : nums)
        {
            if (winning_nums.contains(num))
            {
                winning_count++;
            }
        }
        return winning_count;
    }
};

std::vector<game> parse_input(std::string_view path)
{
    auto file = std::ifstream(path.data());
    auto file_it = std::istream_iterator<std::string>(file);

    auto games = std::vector<game>();

    auto line = std::string();
    while (std::getline(file, line))
    {
        auto winning_start = line.find_first_of(':') + 2;
        auto nums_start = line.find_first_of('|') + 2;

        auto winning_view = std::string_view(line.data() + winning_start, line.data() + nums_start - 3);
        auto nums_view = std::string_view(line.data() + nums_start, line.data() + line.size());

        auto winning_range = winning_view | std::views::split(' ') |
            std::views::filter([](auto&& r) { return !r.empty(); }) |
            std::views::transform(
                [](auto&& r)
                {
                    auto number = 0;
                    std::from_chars(r.begin(), r.end(), number);
                    return number;
                });
        auto winning = std::set(winning_range.begin(), winning_range.end());

        auto numbers_range = nums_view | std::views::split(' ') |
            std::views::filter([](auto&& r) { return !r.empty(); }) |
            std::views::transform(
                [](auto&& r)
                {
                    auto number = 0;
                    std::from_chars(r.begin(), r.end(), number);
                    return number;
                });
        auto numbers = std::vector(numbers_range.begin(), numbers_range.end());

        games.emplace_back(winning, numbers);
    }

    return games;
}

size_t calc_winning_points(const std::vector<game>& games)
{
    size_t points_sum = 0;

    for (const auto& game : games)
    {
        points_sum += game.points();
    }

    return points_sum;
}

size_t calc_num_scratchcards(const std::vector<game>& games)
{
    auto amount_of_plays = std::vector<size_t>();
    amount_of_plays.resize(games.size());
    std::fill(amount_of_plays.begin(), amount_of_plays.end(), 1);

    for (size_t i = 0; i < games.size(); i++)
    {
        const auto& game = games[i];

        size_t wins = game.wins();
        for (size_t k = 1; k <= wins; k++)
        {
            amount_of_plays[i + k] += amount_of_plays[i];
        }
    }

    return std::accumulate(amount_of_plays.begin(), amount_of_plays.end(), 0);
}

int main()
{
    auto games = parse_input("in.txt");

    std::cout << "Points: " << calc_winning_points(games) << '\n';
    std::cout << "Scratchcards: " << calc_num_scratchcards(games) << '\n';
}
