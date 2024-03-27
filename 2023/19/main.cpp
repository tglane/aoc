#include <algorithm>
#include <cassert>
#include <charconv>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

class WorkflowResult
{
public:
    enum Type
    {
        Rejected = 'R',
        Accepted = 'A',
        Workflow = 'W',
    };

    WorkflowResult(Type t, std::optional<std::string> next)
        : m_type{t}
        , m_follow_up{next}
    {}

    WorkflowResult(std::string_view input)
    {
        if (input == "A")
        {
            m_type = Type::Accepted;
            m_follow_up = std::nullopt;
        }
        else if (input == "R")
        {
            m_type = Type::Rejected;
            m_follow_up = std::nullopt;
        }
        else
        {
            m_type = Type::Workflow;
            m_follow_up = std::string(input);
        }
    }

    bool accepted() const
    {
        return m_type == Type::Accepted;
    }

    bool rejected() const
    {
        return m_type == Type::Rejected;
    }

    bool has_next() const
    {
        return m_type == Type::Workflow;
    }

    std::optional<std::string> next_workflow_name() const
    {
        return m_follow_up;
    }

    Type get_type() const
    {
        return m_type;
    }

private:
    Type m_type;
    std::optional<std::string> m_follow_up;
};

enum class Operation : uint8_t
{
    LT = '<',
    GT = '>',
    None,
};

class Rule
{
public:
    char m_c;
    Operation m_op;
    size_t m_comp;
    WorkflowResult m_res;

    Rule(std::string_view input)
        : m_res{WorkflowResult::Rejected, std::nullopt}
    {
        if (input.find(':') != std::string::npos)
        {
            // Real rule
            auto parts = input | std::views::split(std::string_view(":"));
            auto parts_it = parts.begin();

            m_c = (*parts_it).front();
            m_op = static_cast<Operation>((*parts_it)[1]);
            m_comp = 0;
            std::from_chars((*parts_it).begin() + 2, (*parts_it).end(), m_comp);

            std::advance(parts_it, 1);
            m_res = WorkflowResult(std::string_view((*parts_it).begin(), (*parts_it).end()));
        }
        else
        {
            // No condition
            m_op = Operation::None;
            m_res = WorkflowResult(input);
        }
    }

    std::optional<WorkflowResult> operator()(const std::unordered_map<char, size_t>& in) const
    {
        if (auto it = in.find(m_c); it != in.end())
        {
            switch (m_op)
            {
                case Operation::LT:
                    if (it->second < m_comp)
                    {
                        return m_res;
                    }
                    break;
                case Operation::GT:
                    if (it->second > m_comp)
                        return m_res;
                    break;
                case Operation::None:
                    return m_res;
            }
        }
        else if (m_op == Operation::None)
        {
            return m_res;
        }
        return std::nullopt;
    }
};

class Workflow
{
    std::string m_id;
    std::vector<Rule> m_rules;

public:
    Workflow(std::string_view input)
        : m_rules{}
    {
        auto start = input.find('{');
        auto end = input.find('}');

        m_id = std::string(input.begin(), input.begin() + start);

        auto rules = std::string_view(input.begin() + start + 1, input.begin() + end) | std::views::split(',');
        for (auto rule : rules)
        {
            m_rules.emplace_back(std::string_view(rule.begin(), rule.end()));
        }
    }

    WorkflowResult operator()(const std::unordered_map<char, size_t>& in) const
    {
        for (const auto& rule : m_rules)
        {
            auto result = rule(in);
            if (result.has_value())
            {
                return result.value();
            }
        }

        return WorkflowResult(WorkflowResult::Type::Rejected, std::nullopt);
    }

    std::optional<WorkflowResult> fallback_result() const
    {
        for (const auto& rule : m_rules)
        {
            if (rule.m_op == Operation::None)
                return rule.m_res;
        }
        return std::nullopt;
    }

    const std::vector<Rule>& rules() const
    {
        return m_rules;
    }
};

class Pipeline
{
    std::unordered_map<std::string, Workflow> m_workflows;

    WorkflowResult inner(const Workflow& workflow, const std::unordered_map<char, size_t>& in) const
    {
        auto result = workflow(in);
        if (result.has_next())
        {
            result = inner(m_workflows.at(result.next_workflow_name().value()), in);
        }
        return result;
    }

public:
    Pipeline(std::string_view input)
        : m_workflows{}
    {
        auto workflow_lines = input | std::views::split('\n') | std::views::filter([](auto r) { return !r.empty(); });
        for (auto workflow : workflow_lines)
        {
            auto workflow_view = std::string_view(workflow.begin(), workflow.end());
            auto id_end = workflow_view.find('{');
            m_workflows.insert_or_assign(
                std::string(workflow_view.begin(), workflow_view.begin() + id_end), Workflow(workflow_view));
        }
    }

    const std::unordered_map<std::string, Workflow>& workflows() const
    {
        return m_workflows;
    }

    WorkflowResult operator()(const std::unordered_map<char, size_t>& in) const
    {
        auto& workflow = m_workflows.at("in");
        return inner(workflow, in);
    }
};

std::pair<Pipeline, std::vector<std::unordered_map<char, size_t>>> parse_input(std::string_view data)
{
    auto parts = data | std::views::split(std::string_view("\n\n"));

    auto pipeline = Pipeline(std::string_view((*parts.begin()).begin(), (*parts.begin()).end()));

    auto commands = std::vector<std::unordered_map<char, size_t>>();
    auto input_lines = *std::next(parts.begin()) | std::views::split('\n');
    for (auto line : input_lines)
    {
        auto entries = std::ranges::subrange(line.begin() + 1, line.end() - 1) | std::views::split(',') |
            std::views::transform(
                [](auto entry)
                {
                    size_t num = 0;
                    std::from_chars(entry.begin() + 2, entry.end(), num);
                    return std::make_pair(entry.front(), num);
                });

        commands.emplace_back(entries.begin(), entries.end());
    }

    return std::make_pair(std::move(pipeline), std::move(commands));
}

size_t count_configs(const std::unordered_map<std::string, Workflow>& workflows,
    std::unordered_map<char, std::pair<size_t, size_t>> in_ranges,
    WorkflowResult stage = WorkflowResult(WorkflowResult::Type::Workflow, "in"))
{
    if (stage.rejected())
    {
        return 0;
    }
    if (stage.accepted())
    {
        size_t product = 1;
        for (auto& range_pair : in_ranges)
        {
            if (range_pair.second.first == 0)
                range_pair.second.first += 1;
            product *= range_pair.second.second - range_pair.second.first + 1;
        }
        return product;
    }

    const auto& workflow = workflows.at(stage.next_workflow_name().value());

    size_t total = 0;

    for (const auto& rule : workflow.rules())
    {
        std::pair<size_t, size_t> true_half;
        std::pair<size_t, size_t> false_half;

        if (rule.m_op == Operation::None)
        {
            if (auto fallback = workflow.fallback_result(); fallback.has_value())
            {
                total += count_configs(workflows, std::move(in_ranges), fallback.value());
            }
            continue;
        }

        const auto [low, high] = in_ranges.at(rule.m_c);
        if (rule.m_op == Operation::LT)
        {
            true_half = std::make_pair(low, rule.m_comp - 1);
            false_half = std::make_pair(rule.m_comp, high);
        }
        else if (rule.m_op == Operation::GT)
        {
            true_half = std::make_pair(rule.m_comp + 1, high);
            false_half = std::make_pair(low, rule.m_comp);
        }

        if (true_half.first <= true_half.second)
        {
            auto new_in_ranges = in_ranges;
            new_in_ranges[rule.m_c] = true_half;
            total += count_configs(workflows, std::move(new_in_ranges), rule.m_res);
        }

        if (false_half.first <= false_half.second)
        {
            in_ranges[rule.m_c] = false_half;
        }
    }

    return total;
}

void test()
{
    auto input = std::string_view(R"(px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013})");

    auto [pipeline, commands] = parse_input(input);

    size_t passing_sum = 0;
    for (const auto& command : commands)
    {
        auto res = pipeline(command);
        if (res.accepted())
        {
            size_t rating_number = 0;
            for (const auto& pair : command)
            {
                rating_number += pair.second;
            }
            passing_sum += rating_number;
        }
    }
    assert(passing_sum == 19114);

    size_t possible_configs = count_configs(pipeline.workflows(),
        std::unordered_map<char, std::pair<size_t, size_t>>{
            {'x', std::pair(0, 4000)},
            {'m', std::pair(0, 4000)},
            {'a', std::pair(0, 4000)},
            {'s', std::pair(0, 4000)},
        });
    assert(possible_configs == 167409079868000);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto [pipeline, commands] = parse_input(input);

    size_t passing_sum = 0;
    for (const auto& command : commands)
    {
        auto res = pipeline(command);
        if (res.accepted())
        {
            size_t rating_number = 0;
            for (const auto& pair : command)
            {
                rating_number += pair.second;
            }
            passing_sum += rating_number;
        }
    }
    std::cout << "A) Sum of accepted commands: " << passing_sum << '\n';

    size_t possible_configs = count_configs(pipeline.workflows(),
        std::unordered_map<char, std::pair<size_t, size_t>>{
            {'x', std::pair(0, 4000)},
            {'m', std::pair(0, 4000)},
            {'a', std::pair(0, 4000)},
            {'s', std::pair(0, 4000)},
        });
    std::cout << "B) Number of possible configs: " << possible_configs << '\n';
}
