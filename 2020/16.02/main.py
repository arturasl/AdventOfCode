import sys
from dataclasses import dataclass
from typing import TextIO


@dataclass()
class Range:
    start_incl: int
    end_incl: int


@dataclass()
class Rule:
    name: str
    range_disjunction: list[Range]

    def should_accept(self, val: int):
        return any(
            disjunction.start_incl <= val <= disjunction.end_incl
            for disjunction in self.range_disjunction
        )


@dataclass()
class Ticket:
    vals: list[int]


def read_non_empty_line(stream: TextIO) -> str:
    line = ""
    while not line:
        line = stream.readline().strip()
    return line


def read_rules(stream: TextIO) -> dict[str, Rule]:
    rules: dict[str, Rule] = {}
    for line in stream:
        line = line.strip()
        if not line:
            break
        name_and_rule = line.split(": ")
        range_disjunctions: list[Range] = []
        for disjunction in name_and_rule[1].split(" or "):
            parts = disjunction.split("-")
            range_disjunctions.append(Range(int(parts[0]), int(parts[1])))

        name = name_and_rule[0]
        assert name not in rules
        rules[name] = Rule(name, range_disjunctions)
    return rules


def read_your_ticket(stream: TextIO) -> Ticket:
    assert read_non_empty_line(stream) == "your ticket:"
    return Ticket([int(x) for x in read_non_empty_line(stream).split(",")])


def read_nearby_tickets(stream: TextIO) -> list[Ticket]:
    assert read_non_empty_line(stream) == "nearby tickets:"

    tickets: list[Ticket] = []
    for line in stream:
        line = line.strip()
        if not line:
            continue
        tickets.append(Ticket([int(x) for x in line.split(",")]))
    return tickets


def main():
    rules = read_rules(sys.stdin)
    your_ticket = read_your_ticket(sys.stdin)
    nearby_tickets = read_nearby_tickets(sys.stdin)
    nearby_tickets = [
        ticket
        for ticket in nearby_tickets
        if not any(
            not any(rule.should_accept(val) for rule in rules.values())
            for val in ticket.vals
        )
    ]

    field_rule_names: list[str | None] = [None for _ in your_ticket.vals]
    while any(not rule_name for rule_name in field_rule_names):
        for i, _ in enumerate(field_rule_names):
            if field_rule_names[i]:
                continue

            avalable_rule_names = set(rules.keys()) - set(
                rule_name for rule_name in field_rule_names if rule_name
            )
            for ticket in nearby_tickets:
                avalable_rule_names = set(
                    rule_name
                    for rule_name in avalable_rule_names
                    if rules[rule_name].should_accept(ticket.vals[i])
                )

            assert avalable_rule_names
            if len(avalable_rule_names) == 1:
                field_rule_names[i] = avalable_rule_names.pop()

    ans = 1
    for val, rule_name in zip(your_ticket.vals, field_rule_names):
        if rule_name and rule_name.startswith("departure"):
            ans *= val
    print(ans)


if __name__ == "__main__":
    main()
