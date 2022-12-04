#include <cassert>
#include <iostream>
#include <regex>
#include <sstream>
using namespace std;
typedef pair<int, int> pii;

pair<pii, pii> parse(const string &s) {
  static const regex re("(\\d+)-(\\d+),(\\d+)-(\\d+)");
  smatch m;
  assert(regex_match(s, m, re));
  return {{stoi(m[1]), stoi(m[2])}, {stoi(m[3]), stoi(m[4])}};
}

bool is_sub(const pii &l, const pii &r) {
  return !(r.second < l.first || l.second < r.first);
}

int main() {
  ios_base::sync_with_stdio(false), cin.tie(0);
  int r = 0;
  for (string line; getline(cin, line);) {
    const auto [lhs, rhs] = parse(line);
    r += is_sub(lhs, rhs) || is_sub(rhs, lhs);
  }
  cout << r << endl;
  return 0;
}
