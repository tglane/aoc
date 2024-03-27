#include <algorithm>
#include <cassert>
#include <charconv>
#include <fstream>
#include <functional>
#include <iostream>
#include <iterator>
#include <ranges>
#include <string_view>
#include <vector>

enum class HandKind : uint8_t
{
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
};

enum class Card : uint8_t
{
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
};

class Hand
{
    std::vector<Card> m_cards;
    size_t m_bid;
    HandKind m_kind;

    friend bool operator<(const Hand&, const Hand&);

public:
    Hand(std::vector<Card> cards, size_t bid)
        : m_cards{std::move(cards)}
        , m_bid{bid}
    {
        size_t joker_count = 0;
        auto cache = std::vector<size_t>(15);
        for (const auto& card : m_cards)
        {
            if (card == Card::Joker)
            {
                joker_count++;
                continue;
            }
            cache[static_cast<uint8_t>(card)] += 1;
        }

        std::sort(cache.begin(), cache.end(), std::greater<>());

        auto first = cache.begin();
        auto sec = std::next(cache.begin(), 1);

        if (*first == 5 || *first + joker_count >= 5)
        {
            m_kind = HandKind::FiveOfAKind;
        }
        else if (*first == 4 || *first + joker_count >= 4)
        {
            m_kind = HandKind::FourOfAKind;
        }
        else if ((*first == 3 && *sec == 2) || (*first == 2 && *sec == 2 && joker_count >= 1) ||
            (*first == 2 && *sec == 1 && joker_count >= 2) || joker_count == 3)
        {
            // Full house:
            // - 3 + 2
            // - 2 + 2 + 1J
            // - 2 + 1 + 2J
            // - 1 + 1 + 3J
            m_kind = HandKind::FullHouse;
        }
        else if (*first == 3 || *first + joker_count >= 3)
        {
            m_kind = HandKind::ThreeOfAKind;
        }
        else if ((*first == 2 && *sec == 2) || (*first == 2 && joker_count == 1))
        {
            m_kind = HandKind::TwoPair;
        }
        else if (*first == 2 || joker_count == 1)
        {
            m_kind = HandKind::OnePair;
        }
        else
        {
            m_kind = HandKind::HighCard;
        }
    }

    size_t bid() const
    {
        return m_bid;
    }
};

bool operator<(const Hand& lhs, const Hand& rhs)
{
    if (lhs.m_kind != rhs.m_kind)
    {
        return lhs.m_kind < rhs.m_kind;
    }

    // If we have the same kind, we need to iterate through the cards and compare them
    for (size_t i = 0; i < lhs.m_cards.size(); i++)
    {
        if (lhs.m_cards[i] != rhs.m_cards[i])
        {
            return lhs.m_cards[i] < rhs.m_cards[i];
        }
    }

    return false;
}

std::vector<Hand> parse_input(std::string_view data, bool with_joker)
{
    auto parsed = std::vector<Hand>();

    auto hand_ranges = data | std::views::split('\n') | std::views::filter([](auto&& r) { return !r.empty(); });

    for (auto&& hand_range : hand_ranges)
    {
        auto separator = std::ranges::search(hand_range, std::string_view(" "));
        auto cards_view = std::string_view(hand_range.begin(), separator.begin());
        auto bid_view = std::string_view(separator.begin() + 1, hand_range.end());

        auto card_range = cards_view |
            std::views::transform(
                [with_joker](char c)
                {
                    switch (c)
                    {
                        case 'A':
                            return Card::A;
                        case 'K':
                            return Card::K;
                        case 'Q':
                            return Card::Q;
                        case 'J':
                            if (with_joker)
                                return Card::Joker;
                            else
                                return Card::J;
                        case 'T':
                            return Card::T;
                        default:
                            return static_cast<Card>(c - 48);
                    }
                });
        auto cards = std::vector<Card>(card_range.begin(), card_range.end());

        size_t bid = 0;
        std::from_chars(bid_view.begin(), bid_view.end(), bid);

        parsed.emplace_back(cards, bid);
    }

    return parsed;
}

size_t calc_rank_sum(std::vector<Hand>& hands)
{
    std::sort(hands.begin(), hands.end());

    size_t rank_sum = 0;
    for (size_t i = 0; i < hands.size(); i++)
    {
        rank_sum += (i + 1) * hands[i].bid();
    }

    return rank_sum;
}

void test()
{
    auto input = std::string_view(R"(32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483)");

    auto hands = parse_input(input, false);
    auto rank_sum = calc_rank_sum(hands);
    std::cout << "[Test] The sum of ranks: " << rank_sum << '\n';
    assert(rank_sum == 6440);

    auto hands_with_joker = parse_input(input, true);
    auto rank_sum_with_joker = calc_rank_sum(hands_with_joker);
    std::cout << "[Test] The sum of ranks with joker: " << rank_sum_with_joker << '\n';
    // assert(rank_sum_with_joker == 5905);
}

int main()
{
    test();

    auto file = std::ifstream("in.txt");
    auto input_string = std::string(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());

    auto hands = parse_input(input_string, false);
    auto rank_sum = calc_rank_sum(hands);
    std::cout << "The sum of ranks: " << rank_sum << '\n';

    auto hands_with_joker = parse_input(input_string, true);
    auto rank_sum_with_joker = calc_rank_sum(hands_with_joker);
    std::cout << "The sum of ranks with joker: " << rank_sum_with_joker << '\n';
}
