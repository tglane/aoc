#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <ranges>
#include <set>
#include <string>
#include <string_view>

std::map<std::string, std::set<std::string>> parse_input(std::string_view data)
{
    auto nodes = std::map<std::string, std::set<std::string>>();

    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    for (auto&& line : lines)
    {
        auto parts = line | std::views::split(std::string_view(": "));
        for (auto&& dest : *std::next(parts.begin()) | std::views::split(' '))
        {
            auto a = std::string(parts.front().begin(), parts.front().end());
            auto b = std::string(dest.begin(), dest.end());

            if (auto it = nodes.find(a); it != nodes.end())
                it->second.insert(b);
            else
                nodes.insert_or_assign(a, std::set{b});

            if (auto it = nodes.find(b); it != nodes.end())
                it->second.insert(a);
            else
                nodes.insert_or_assign(b, std::set{a});
        }
    }

    return nodes;
}

std::pair<size_t, size_t> subgroup_sizes(const std::map<std::string, std::set<std::string>>& nodes)
{
    auto subgroup = std::set<std::string>();
    for (const auto& p : nodes)
    {
        subgroup.insert(p.first);
    }

    const auto count = [&nodes, &subgroup](const std::string& v)
    {
        size_t from_v_without_s = 0;
        for (const auto& to : nodes.at(v))
        {
            if (auto it = subgroup.find(to); it == subgroup.end())
                from_v_without_s++;
        }
        return from_v_without_s;
    };
    const auto map_sum = [&count](const std::set<std::string>& s)
    {
        size_t sum = 0;
        for (const auto& n : s)
        {
            sum += count(n);
        }
        return sum;
    };

    while (map_sum(subgroup) != 3)
    {
        auto to_be_erased = std::max_element(
            subgroup.begin(), subgroup.end(), [&count](const auto& a, const auto& b) { return count(a) < count(b); });
        subgroup.erase(*to_be_erased);
    }

    size_t nodes_without_subgroup = 0;
    for (const auto& node : nodes)
    {
        if (auto it = subgroup.find(node.first); it == subgroup.end())
            nodes_without_subgroup++;
    }
    return std::make_pair(subgroup.size(), nodes_without_subgroup);
}

int main()
{
    auto input_t = std::string_view(R"(jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr)");

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto adjacency_mat = parse_input(input);
    // for (const auto& e : edges)
    // {
    //     std::cout << "From " << e.a << " to " << e.b << '\n';
    // }
    const auto [size_a, size_b] = subgroup_sizes(adjacency_mat);
    std::cout << "A) Product of subgraph sizes: " << size_a * size_b << '\n';
}
