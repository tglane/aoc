#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using cave_map = std::unordered_map<std::string_view, std::vector<std::string>>;
using recent_path = std::unordered_set<std::string>;

struct rule
{
    rule(const std::string& from, const std::string& to)
        : from{from}
        , to{to}
    {}

    std::string from;
    std::string to;
};

std::vector<rule> parse_input(std::string_view filename)
{
    std::fstream ifs{filename.data()};

    std::vector<rule> out;
    std::string line;
    while(std::getline(ifs, line))
    {
        size_t delimiter_pos = line.find('-');
        out.emplace_back(line.substr(0, delimiter_pos), line.substr(delimiter_pos + 1, line.size() - delimiter_pos));
    }
    return out;
}

cave_map build_map(const std::vector<rule>& rules)
{
    cave_map map;
    for(const auto& rule : rules)
    {
        map[rule.from].push_back(rule.to);
        map[rule.to].push_back(rule.from);
    }
    return map;
}

size_t step(std::string_view curr_cave, const cave_map& map, recent_path recent, bool double_checked)
{
    if(curr_cave == "end")
    {
        return 1;
    }
    else
    {
        size_t new_paths = 0;
        for(const auto& next_cave : map.at(curr_cave))
        {
            if(next_cave != "start")
            {
                if(std::all_of(next_cave.begin(), next_cave.end(), [](const char c) { return isupper(c); }) ||
                    recent.find(next_cave) == recent.end())
                {
                    recent_path recent_clone = recent;
                    recent_clone.insert(next_cave);
                    new_paths += step(next_cave, map, recent_clone, double_checked);
                }
                else if(!double_checked)
                {
                    new_paths += step(next_cave, map, recent, true);
                }
            }
        }
        return new_paths;
    }
}

void one(const cave_map& map)
{
    size_t path_count = step("start", map, {}, true);
    std::cout << "TWO: Paths from start to end: " << path_count << '\n';
}

void two(const cave_map& map)
{
    size_t path_count = step("start", map, {}, false);
    std::cout << "TWO: Paths from start to end: " << path_count << '\n';
}

int main()
{
    auto rules = parse_input("in.txt");
    auto map = build_map(rules);

    one(map);
    two(map);
}
