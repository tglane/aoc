#include <algorithm>
#include <cassert>
#include <charconv>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <ranges>
#include <set>
#include <string_view>
#include <vector>

struct Point
{
    int64_t x;
    int64_t y;
    int64_t z;

    auto operator<=>(const Point&) const = default;
};

class Brick
{
    Point m_a;
    Point m_b;

public:
    Brick(const Point& a, const Point& b)
        : m_a{a}
        , m_b{b}
    {}

    int64_t bottom_z() const
    {
        return std::min(m_a.z, m_b.z);
    }

    int64_t top_z() const
    {
        return std::max(m_a.z, m_b.z);
    }

    void set_z(int64_t z)
    {
        m_b.z -= m_a.z - z;
        m_a.z = z;
    }

    bool z_less_than(const Brick& other) const
    {
        // Search lower z point and compare those
        return std::min(m_a.z, m_b.z) < std::min(other.m_a.z, other.m_b.z);
    }

    bool x_y_overlap(const Brick& other) const
    {
        // (    [ )    ]
        // max(s1, s2) <= min(e1, e2)
        return (std::max(m_a.x, other.m_a.x) <= std::min(m_b.x, other.m_b.x)) &&
            (std::max(m_a.y, other.m_a.y) <= std::min(m_b.y, other.m_b.y));
    }
};

std::vector<Brick> parse_input(std::string_view snapshot)
{
    auto bricks = std::vector<Brick>();
    auto lines = snapshot | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
    bricks.reserve(std::distance(lines.begin(), lines.end()));
    for (auto&& line : lines)
    {
        auto points = line | std::views::split('~') |
            std::views::transform(
                [](auto r)
                {
                    auto coords = r | std::views::split(',');
                    auto coords_it = coords.begin();
                    assert(std::distance(coords_it, coords.end()) == 3);

                    int64_t x = 0;
                    std::from_chars((*coords_it).begin(), (*coords_it).end(), x);
                    std::advance(coords_it, 1);

                    int64_t y = 0;
                    std::from_chars((*coords_it).begin(), (*coords_it).end(), y);
                    std::advance(coords_it, 1);

                    int64_t z = 0;
                    std::from_chars((*coords_it).begin(), (*coords_it).end(), z);

                    return Point{x, y, z};
                });

        bricks.emplace_back(*points.begin(), *std::next(points.begin()));
    }
    return bricks;
}

std::pair<std::map<size_t, std::set<size_t>>, std::map<size_t, std::set<size_t>>> precompute_bricks(
    std::vector<Brick>& bricks)
{
    // Sort by z axis
    std::ranges::sort(bricks, [](const auto& a, const auto& b) { return a.z_less_than(b); });

    // Drop bricks
    for (size_t i = 0; i < bricks.size(); i++)
    {
        auto& brick = bricks[i];
        int64_t new_z = 1;

        for (size_t j = 0; j < i; j++)
        {
            if (brick.x_y_overlap(bricks[j]))
            {
                new_z = std::max(new_z, bricks[j].top_z() + 1);
            }
        }
        brick.set_z(new_z);
    }

    // Sort by z axis after dropping bricks
    std::ranges::sort(bricks, [](const auto& a, const auto& b) { return a.z_less_than(b); });

    // Get support structure of bricks
    auto supports = std::map<size_t, std::set<size_t>>();
    auto is_supported = std::map<size_t, std::set<size_t>>();
    for (size_t j = 0; j < bricks.size(); j++)
    {
        for (size_t i = 0; i < j; i++)
        {
            // j == upper brick
            // i == lower brick
            if (bricks[i].x_y_overlap(bricks[j]) && bricks[j].bottom_z() == bricks[i].top_z() + 1)
            {
                supports[i].insert(j);
                is_supported[j].insert(i);
            }
        }
    }

    return std::make_pair(supports, is_supported);
}

size_t count_disintegratable_bricks(std::vector<Brick> bricks)
{
    auto pair = precompute_bricks(bricks);
    auto& supports = pair.first;
    auto& is_supported = pair.second;

    size_t disintegratable = 0;
    for (size_t i = 0; i < bricks.size(); i++)
    {
        auto& supports_i = supports[i];
        if (std::all_of(supports_i.begin(),
                supports_i.end(),
                [&is_supported](size_t j) { return is_supported.at(j).size() >= 2; }))
        {
            disintegratable++;
        }
    }
    return disintegratable;
}

size_t total_bricks_falling(std::vector<Brick> bricks)
{
    auto pair = precompute_bricks(bricks);
    auto& supports = pair.first;
    auto& is_supported = pair.second;

    size_t total_falling = 0;

    // Disintegrate a brick b causes other bricks (the one that are supported by b)
    // Disintegrate every brick and check how many other bricks would fall as a chain reaction
    // Sum those bricks up

    for (size_t i = 0; i < bricks.size(); i++)
    // for (size_t i = 0; i < 1; i++)
    {
        auto q = std::deque<size_t>();
        auto falling = std::set<size_t>();
        for (size_t j : supports[i])
        {
            if (is_supported[j].size() == 1)
            {
                q.push_back(j);
                falling.insert(j);
            }
        }
        falling.insert(i);

        while (!q.empty())
        {
            size_t j = q.front();
            q.pop_front();

            for (size_t k : supports[j] | std::views::filter([&falling](auto s) { return !falling.contains(s); }))
            {
                const auto& k_is_supported_by = is_supported[k];
                if (std::all_of(k_is_supported_by.begin(),
                        k_is_supported_by.end(),
                        [&falling](auto s) { return falling.contains(s); }))
                {
                    q.push_back(k);
                    falling.insert(k);
                }
            }
        }

        total_falling += falling.size() - 1;
    }

    return total_falling;
}

void test()
{
    const auto input = std::string_view(R"(1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9)");

    const auto bricks = parse_input(input);
    size_t disintegratable = count_disintegratable_bricks(bricks);
    assert(disintegratable == 5);

    size_t chain_reactions = total_bricks_falling(bricks);
    assert(chain_reactions == 7);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    const auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    const auto bricks = parse_input(input);
    size_t disintegratable = count_disintegratable_bricks(bricks);
    std::cout << "A) Number of disintegratable bricks: " << disintegratable << '\n';

    size_t chain_reactions = total_bricks_falling(bricks);
    std::cout << "B) Number of bricks that would fall through chain reactions: " << chain_reactions << '\n';
}
