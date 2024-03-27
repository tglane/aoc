#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <memory>
#include <numeric>
#include <optional>
#include <queue>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

enum class Pulse : uint8_t
{
    Low = '-',
    High = '+',
};

struct Module
{
    std::vector<std::string> m_next;

    Module(std::vector<std::string> next)
        : m_next{std::move(next)}
    {}

    virtual ~Module(){};
    virtual std::pair<Pulse, std::optional<std::span<std::string>>> send(Pulse p, const std::string& from) = 0;
};

struct Broadcaster : public Module
{
    Broadcaster(std::vector<std::string> next)
        : Module{std::move(next)}
    {}

    std::pair<Pulse, std::optional<std::span<std::string>>> send(Pulse p, const std::string&) override
    {
        return std::make_pair(p, std::span(m_next));
    }
};

struct FlipFlop : public Module
{
    bool m_state = false;

    FlipFlop(std::vector<std::string> next)
        : Module{std::move(next)}
    {}

    std::pair<Pulse, std::optional<std::span<std::string>>> send(Pulse p, const std::string&)
    {
        if (p == Pulse::Low)
        {
            m_state = !m_state;
            return std::make_pair(m_state ? Pulse::High : Pulse::Low, std::span(m_next));
        }
        return std::make_pair(m_state ? Pulse::High : Pulse::Low, std::nullopt);
    }
};

struct Conjuction : public Module
{
    std::unordered_map<std::string, Pulse> m_inputs;

    Conjuction(std::vector<std::string> next)
        : Module{std::move(next)}
        , m_inputs{}
    {}

    void add_input(const std::string& in)
    {
        m_inputs.insert_or_assign(in, Pulse::Low);
    }

    std::pair<Pulse, std::optional<std::span<std::string>>> send(Pulse p, const std::string& from) override
    {
        m_inputs.insert_or_assign(from, p);

        if (std::all_of(m_inputs.begin(), m_inputs.end(), [](const auto& p) { return p.second == Pulse::High; }))
            return std::make_pair(Pulse::Low, std::span(m_next));
        else
            return std::make_pair(Pulse::High, std::span(m_next));
    }
};

std::unordered_map<std::string, std::unique_ptr<Module>> parse_input(std::string_view data)
{
    auto lines = data | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });

    auto modules = std::unordered_map<std::string, std::unique_ptr<Module>>();
    for (auto line : lines)
    {
        auto parts = line | std::views::split(std::string_view(" -> "));
        auto next = *std::next(parts.begin()) | std::views::split(std::string_view(", ")) |
            std::views::transform([](auto r) { return std::string(r.begin(), r.end()); });
        auto next_vec = std::vector(next.begin(), next.end());

        switch (line.front())
        {
            case '%': {
                auto id = std::string((*parts.begin()).begin() + 1, (*parts.begin()).end());
                modules.insert(std::make_pair(std::move(id), std::make_unique<FlipFlop>(std::move(next_vec))));
                break;
            }
            case '&': {
                auto id = std::string((*parts.begin()).begin() + 1, (*parts.begin()).end());
                modules.insert(std::make_pair(std::move(id), std::make_unique<Conjuction>(std::move(next_vec))));
                break;
            }
            default: {
                auto id = std::string((*parts.begin()).begin(), (*parts.begin()).end());
                modules.insert(std::make_pair(std::move(id), std::make_unique<Broadcaster>(std::move(next_vec))));
                break;
            }
        }
    }

    for (const auto& p : modules)
    {
        for (const auto& m : p.second->m_next)
        {
            if (auto out_it = modules.find(m); out_it != modules.end())
            {
                Conjuction* cp = dynamic_cast<Conjuction*>(out_it->second.get());
                if (cp != nullptr)
                {
                    cp->add_input(p.first);
                }
            }
        }
    }

    return modules;
}

size_t part_one(const std::unordered_map<std::string, std::unique_ptr<Module>>& modules)
{
    size_t high = 0;
    size_t low = 0;

    for (size_t i = 0; i < 1000; i++)
    {
        // origin, target, pulse
        auto q = std::queue<std::tuple<std::string, std::string, Pulse>>();
        q.push(std::make_tuple("initial", "broadcaster", Pulse::Low));

        while (!q.empty())
        {
            const auto [origin, target, pulse] = q.front();
            q.pop();

            if (pulse == Pulse::Low)
                low++;
            else
                high++;

            if (auto target_it = modules.find(target); target_it != modules.end())
            {
                auto [target_pulse, targets_out] = target_it->second->send(pulse, origin);
                if (targets_out.has_value())
                {
                    for (const auto& out : targets_out.value())
                    {
                        q.push(std::make_tuple(target, out, target_pulse));
                    }
                }
            }
        }
    }

    return high * low;
}

size_t part_two(const std::unordered_map<std::string, std::unique_ptr<Module>>& modules)
{
    // &kj -> rx
    // 4 Conjunctions lead to kj
    // feed = kj
    std::string feed;
    for (const auto& m : modules)
    {
        if (auto it = std::find_if(m.second->m_next.begin(),
                m.second->m_next.end(),
                [](const auto& name) { return name == std::string_view("rx"); });
            it != m.second->m_next.end())
        {
            feed = m.first;
            break;
        }
    }

    // Track cycles to reach kj
    auto cycle_lengths = std::unordered_map<std::string, size_t>();
    auto seen = std::unordered_map<std::string, size_t>();
    for (const auto& m : modules)
    {
        if (std::any_of(
                m.second->m_next.begin(), m.second->m_next.end(), [feed](const auto& name) { return name == feed; }))
        {
            seen.insert_or_assign(m.first, 0);
        }
    }

    // We already made 1000 button presses in part one
    size_t button_pressed = 1000;

    while (true)
    {
        button_pressed++;

        // origin, target, pulse
        auto q = std::queue<std::tuple<std::string, std::string, Pulse>>();
        q.push(std::make_tuple("initial", "broadcaster", Pulse::Low));

        while (!q.empty())
        {
            const auto [origin, target, pulse] = q.front();
            q.pop();

            if (target == feed && pulse == Pulse::High)
            {
                // We need to pass a high pulse into the feed module in order to send a low pulse to the destination
                seen[origin] += 1;

                if (auto it = cycle_lengths.find(origin); it == cycle_lengths.end())
                {
                    cycle_lengths.insert(std::make_pair(origin, button_pressed));
                }
                else
                {
                    assert(button_pressed == seen.at(origin) * cycle_lengths.at(origin));
                }

                if (std::all_of(seen.begin(), seen.end(), [](const auto& p) { return p.second > 0; }))
                {
                    size_t x = 1;
                    for (const auto& cl : cycle_lengths)
                    {
                        x = std::lcm(x, cl.second);
                    }
                    return x;
                }
            }

            if (auto target_it = modules.find(target); target_it != modules.end())
            {
                auto [target_pulse, targets_out] = target_it->second->send(pulse, origin);
                if (targets_out.has_value())
                {
                    for (const auto& out : targets_out.value())
                    {
                        q.push(std::make_tuple(target, out, target_pulse));
                    }
                }
            }
        }
    }

    // return button_pressed;
    return 0;
}

int main()
{
    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto module_config = parse_input(input);
    size_t pulses = part_one(module_config);
    std::cout << "A) Product of high and low pulses: " << pulses << '\n';

    size_t pressed = part_two(module_config);
    std::cout << "B) Number of button presses to send low pulse to RX: " << pressed << '\n';
}
