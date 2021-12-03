#include <fstream>
#include <iostream>
#include <iterator>
#include <string_view>
#include <vector>

std::vector<int> file_to_vec(std::string_view filename)
{
    std::fstream ifs{filename.data()};
    return std::vector<int>{std::istream_iterator<int>{ifs}, std::istream_iterator<int>{}};
}

void one(const std::vector<int> &input)
{
    int increased = 0;
    int decreased = 0;
    for (size_t i = 1; i < input.size(); ++i)
    {
        // std::cout << "Comparing " << input[i] << " with " << input[i - 1] << '\n';

        if (input[i] > input[i - 1])
            increased++;
        else
            decreased++;
    }

    std::cout << "ONE: Increased: " << increased << " - Decreased: " << decreased << '\n';
}

void two(const std::vector<int> &input)
{
    int increased = 0;
    int decreased = 0;
    size_t last_window = input[0] + input[1] + input[2];
    for (size_t i = 1; i < input.size() - 2; ++i)
    {
        size_t curr_window = last_window - input[i - 1] + input[i + 2];

        // std::cout << "Comparing " << curr_window << " with " << last_window << '\n';

        if (curr_window > last_window)
            increased++;
        else
            decreased++;

        last_window = curr_window;
    }

    std::cout << "TWO: Increased: " << increased << " - Decreased: " << decreased << '\n';
}

int main()
{
    std::vector<int> input = file_to_vec("in.txt");

    one(input);
    two(input);
}
