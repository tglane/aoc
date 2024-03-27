#include <cassert>
#include <charconv>
#include <format>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <string_view>
#include <vector>

#include "z3++.h"

class Hailstone
{
public:
    int64_t m_x;
    int64_t m_y;
    int64_t m_z;

    int64_t m_vx;
    int64_t m_vy;
    int64_t m_vz;

    Hailstone(int64_t x, int64_t y, int64_t z, int64_t vx, int64_t vy, int64_t vz)
        : m_x{x}
        , m_y{y}
        , m_z{z}
        , m_vx{vx}
        , m_vy{vy}
        , m_vz{vz}
    {}

    void print() const
    {
        std::cout << "X: " << m_x << ", Y: " << m_y << ", Z: " << m_z << '\n';
    }

    void print_vels() const
    {
        std::cout << "VX: " << m_vx << ", VY: " << m_vy << ", VZ: " << m_vz << '\n';
    }

    std::optional<std::pair<double, double>> intersection(const Hailstone& other) const
    {
        // Standard form of a line: ax + by = c
        // Point in line: (x, y) + t * (vx, vy)
        //
        // px = x + tvx
        // py = y + tvy
        //
        // t = (px - x) / vx
        // t = (py - y) / vy
        //
        // vy (px - x) = vx (py - y)
        // vy px - vx py = vy x - vx y
        //
        // a = vy
        // b = -vx
        // c = vy * x - vx * y

        if (m_vy * -other.m_vx == -m_vx * other.m_vy)
        {
            // Parallel
            return std::nullopt;
        }

        double c = (m_vy * m_x) - (m_vx * m_y);
        double other_c = (other.m_vy * other.m_x) - (other.m_vx * other.m_y);

        double x = ((c * -other.m_vx) - (other_c * -m_vx)) / ((m_vy * -other.m_vx - other.m_vy * -m_vx));
        double y = ((other_c * m_vy) - (c * other.m_vy)) / ((m_vy * -other.m_vx - other.m_vy * -m_vx));

        return std::make_pair(x, y);
    }
};

std::vector<Hailstone> parse_input(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });

    auto hailstones = std::vector<Hailstone>();
    for (auto&& line : lines)
    {
        auto parts = line | std::views::split(std::string_view(" @ "));

        auto parts_it = parts.begin();
        auto coords_range = *parts_it | std::views::split(',') |
            std::views::transform(
                [](auto r)
                {
                    int64_t num = 0;
                    auto rsv = std::string_view(r.begin(), r.end());
                    rsv.remove_prefix(std::min(rsv.find_first_not_of(' '), rsv.size()));
                    std::from_chars(rsv.begin(), rsv.end(), num);
                    return num;
                });

        std::advance(parts_it, 1);

        auto vels_range = *parts_it | std::views::split(',') |
            std::views::transform(
                [](auto r)
                {
                    int64_t num = 0;
                    auto rsv = std::string_view(r.begin(), r.end());
                    rsv.remove_prefix(std::min(rsv.find_first_not_of(' '), rsv.size()));
                    std::from_chars(rsv.begin(), rsv.end(), num);
                    return num;
                });

        hailstones.emplace_back(*coords_range.begin(),
            *std::next(coords_range.begin(), 1),
            *std::next(coords_range.begin(), 2),
            *vels_range.begin(),
            *std::next(vels_range.begin(), 1),
            *std::next(vels_range.begin(), 2));
    }

    return hailstones;
}

size_t total_collisions(const std::vector<Hailstone>& hailstones, int64_t min, int64_t max)
{
    size_t collisions = 0;

    for (size_t i = 0; i < hailstones.size(); i++)
    {
        for (size_t j = i + 1; j < hailstones.size(); j++)
        {
            // Check for collisions of i and j in a test area with x and y between 200000000000000 and 400000000000000
            if (auto intersection = hailstones[i].intersection(hailstones[j]); intersection.has_value())
            {
                // std::cout << "Intersection at " << intersection->first << ',' << intersection->second << '\n';
                if (min <= intersection->first && intersection->first <= max && min <= intersection->second &&
                    intersection->second <= max)
                {

                    // Check if intersection is "in the past"
                    // vx (px - x) >= 0
                    // vy (py - y) >= 0
                    if (hailstones[i].m_vx * (intersection->first - hailstones[i].m_x) >= 0 &&
                        hailstones[i].m_vy * (intersection->second - hailstones[i].m_y) >= 0 &&
                        hailstones[j].m_vx * (intersection->first - hailstones[j].m_x) >= 0 &&
                        hailstones[j].m_vy * (intersection->second - hailstones[j].m_y) >= 0)
                    {
                        collisions++;
                    }
                }
            }
        }
    }

    return collisions;
}

std::tuple<int64_t, int64_t, int64_t> rock_position(const std::vector<Hailstone>& hailstones)
{
    auto ctx = z3::context();

    z3::expr xr = ctx.int_const("xr");
    z3::expr yr = ctx.int_const("yr");
    z3::expr zr = ctx.int_const("zr");
    z3::expr vxr = ctx.int_const("vxr");
    z3::expr vyr = ctx.int_const("vyr");
    z3::expr vzr = ctx.int_const("vzr");

    auto solv = z3::solver(ctx);

    size_t i = 0;
    for (const auto& h : hailstones)
    {
        auto x = ctx.int_val(h.m_x);
        auto y = ctx.int_val(h.m_y);
        auto z = ctx.int_val(h.m_z);
        auto vx = ctx.int_val(h.m_vx);
        auto vy = ctx.int_val(h.m_vy);
        auto vz = ctx.int_val(h.m_vz);
        auto t = ctx.int_const(std::format("t{}", i).c_str());

        //      ((xr - x) * (vy - vyr) - (yr - y) * (vx - vxr))
        // solv.add((xr - x) * (vy - vyr) - (yr - y) * (vx - vxr) == 0);
        //      ((yr - y) * (vz - vzr) - (zr - z) * (vy - vyr))
        // solv.add((yr - y) * (vz - vzr) - (zr - z) * (vy - vyr) == 0);

        solv.add(t >= 0);
        solv.add(xr + vxr * t == t * vx + x);
        solv.add(yr + vyr * t == t * vy + y);
        solv.add(zr + vzr * t == t * vz + z);

        if (i++ >= 3)
            break;
    }

    auto satisfied = solv.check();
    std::cout << satisfied << '\n';
    auto m = solv.get_model();

    auto rx = m.eval(xr).as_int64();
    auto ry = m.eval(yr).as_int64();
    auto rz = m.eval(zr).as_int64();

    return std::make_tuple(rx, ry, rz);
}

void test()
{
    auto input = std::string_view(R"(19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3)");

    auto hailstones = parse_input(input);

    size_t collisions = total_collisions(hailstones, 7, 27);
    assert(collisions == 2);

    auto [rx, ry, rz] = rock_position(hailstones);
    assert(rx + ry + rz == 47);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto hailstones = parse_input(input);
    // for (const auto& h : hailstones)
    // {
    //     h.print();
    //     h.print_vels();
    // }

    size_t collisions = total_collisions(hailstones, 200000000000000, 400000000000000);
    std::cout << "A) Total number of collisions in the area: " << collisions << '\n';

    auto [rx, ry, rz] = rock_position(hailstones);
    std::cout << "B) Sum of rock coordinate: " << rx + ry + rz << '\n';
}
