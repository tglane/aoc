#include <charconv>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <string_view>
#include <vector>

struct command
{
    std::string direction;
    size_t count;
};

std::vector<command> file_to_vec(std::string_view filename)
{
    std::vector<command> out;
    std::fstream ifs{filename.data()};
    std::string line;
    while(std::getline(ifs, line))
    {
        size_t delimiter_pos = line.find(' ');
        size_t count;
        auto [_, ec] =
            std::from_chars(line.data() + delimiter_pos + 1, line.data() + line.size() - delimiter_pos, count);
        if(ec != std::errc() || delimiter_pos == std::string::npos)
        {
            throw std::runtime_error{"Failed to parse line."};
        }
        out.push_back({line.substr(0, delimiter_pos), count});
    }

    return out;
}

void one(const std::vector<command> &input)
{
    size_t pos = 0;
    size_t depth = 0;
    for(const auto& it : input)
    {
        if(it.direction == "forward")
        {
            pos += it.count;
        }
        else if(it.direction == "up")
        {
            depth -= it.count;
        }
        else if(it.direction == "down")
        {
            depth += it.count;
        }
    }

    std::cout << "ONE: Pos: " << pos << " - Depth: " << depth << " => " << pos * depth << '\n';
}

void two(const std::vector<command> &input)
{
    size_t pos = 0;
    size_t depth = 0;
    size_t aim = 0;
    for(const auto& it : input)
    {
        if(it.direction == "forward")
        {
            pos += it.count;
            depth += aim * it.count;
        }
        else if(it.direction == "up")
        {
            aim -= it.count;
        }
        else if(it.direction == "down")
        {
            aim += it.count;
        }
    }

    std::cout << "TWO: Pos: " << pos << " - Depth: " << depth << " => " << pos * depth << '\n';
}

int main()
{
    std::vector<command> input = file_to_vec("in.txt");

    one(input);
    two(input);
}
