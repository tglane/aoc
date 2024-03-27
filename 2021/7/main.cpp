#include <algorithm>
#include <charconv>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <optional>
#include <string_view>
#include <vector>

std::vector<int> parse_input(std::string_view filename) {
    std::ifstream ifs {filename.data()};

    std::vector<int> out{};
    std::string str_num;
    while(std::getline(ifs, str_num, ','))
    {
        int num;
        std::from_chars(str_num.data(), str_num.data() + str_num.size(), num);
        out.push_back(num);
    }

  return out;
}

template <typename COST_FUNCTION>
void min_alignment_cost(const std::vector<int>& input, COST_FUNCTION&& cost)
{
    auto max_iter = std::max_element(input.begin(), input.end());
    std::optional<std::pair<size_t, int>> min_cost = std::nullopt;

    for(size_t i = 0; i < *max_iter; ++i)
    {
        int cost_count = 0;
        for(const auto num : input)
        {
            cost_count += cost(std::abs(static_cast<int>(num - i)));
        }

        if(!min_cost || (min_cost && std::get<1>(*min_cost) > cost_count))
        {
            min_cost = {i, cost_count};
        }
    }

    std::cout << "Least cost is " << std::get<1>(*min_cost) << " at " << std::get<0>(*min_cost) << '\n';
}

int main() {
    std::vector<int> input = parse_input("in.txt");

    min_alignment_cost(input, [](int dist) { return dist; });
    min_alignment_cost(input, [](int dist) { return (dist * (dist + 1)) / 2; });
}
