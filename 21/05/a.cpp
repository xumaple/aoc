#include <iostream>
#include <fstream>
#include <map>
#include <unordered_map>
#include <vector>
#include <string>
#include <algorithm>

typedef int coord;
#define next_coord(C) (C+1)
typedef std::pair<coord, coord> interval;
constexpr const coord default_coord_c = -1;
constexpr const coord coord_max = 999999999;

class Point {
public:
    Point(): _x(default_coord_c), _y(default_coord_c) {}
    Point(coord x, coord y): _x(x), _y(y) {}
    Point(std::ifstream &is) {
        char c;
        is >> _x >> c >> _y;
    }

    coord x() const { return _x; }
    coord y() const { return _y; }

    bool operator==(const Point &other) { return _x == other.x() && _y == other.y(); }
    bool operator!=(const Point &other) { return !(*this == other); }

    // friend class Line;
    // friend std::ostream &operator<<(std::ostream &, const Point &);

private:
    coord _x, _y;
} DefaultPoint;

std::ostream &operator<<(std::ostream &os, const Point &p) {
    os << '(' << p.x() << ',' << p.y() << ')';
    return os;
}

class Line {
public:
    Line(std::ifstream &is) {
        char c;
        a = Point(is);
        is >> c >> c;
        b = Point(is);
        is.get(c);

        init();
    }

    bool is_horizontal() const { return h; }
    bool is_vertical() const { return v; }

    const interval &get_interval() const { return interval_range; }
    const coord get_coord() const { return interval_coord; }

private:
    Point a, b;

    bool h, v;
    coord interval_coord;
    interval interval_range;
    void init() {
        coord interval_a, interval_b;
        // std::cout << "init: " << a << " -> " << b << std::endl;
        h = a.y()== b.y();
        v = a.x() == b.x() && !h;

        if (h) {
            interval_coord = a.y();
            interval_a = a.x();
            interval_b = b.x();
        }
        if (v) {
            interval_coord = a.x();
            interval_a = a.y();
            interval_b = b.y();
        }

        if (interval_a > interval_b) std::swap(interval_a, interval_b);
        interval_b = next_coord(interval_b);

        interval_range = { interval_a, interval_b };
        // std::cout << interval_coord << ": " << interval_a << "->" << interval_b << "\n\n";
    }
};

bool in_interval(const coord c, const interval &i) {
    return c >= i.first && c < i.second;
}

const Point &&lines_cross(const Line a, const Line b) {
    if (a.is_vertical() == b.is_vertical() || a.is_horizontal() == b.is_horizontal()) return std::move(Point());
    
    if (!in_interval(a.get_coord(), b.get_interval()) || !in_interval(b.get_coord(), a.get_interval())) return std::move(Point());

    return std::move(a.is_vertical() ? Point(a.get_coord(), b.get_coord()) : Point(b.get_coord(), a.get_coord()));
}

class NumberLine {
public:
    NumberLine() {
        intervals[0] = 0;
        intervals[coord_max] = 0;
    }

    void add_interval(const interval &i) {
        coord a = i.first, b = i.second; // [a, b)
        // std::cout << a << " " << b << std::endl;
        auto left = intervals.insert({a, default_coord_c});
        auto right = intervals.insert({b, default_coord_c});

        auto left_it = left.first;
        auto right_it = right.first;
        if (left.second) {
            left_it->second = std::prev(left_it)->second;
        }
        if (right.second) {
            right_it->second = std::prev(right_it)->second;
        }

        while (left_it != right_it) {
            (left_it++)->second++;
        }

        // print_intervals();
    }

    void zero_coord(const coord c) {
        // std::cout << "zeroing " << c << std::endl;
        intervals.insert({next_coord(c), // if next_coord(c) already exists then do nothing
            std::prev(std::upper_bound(intervals.begin(), 
                                       intervals.end(), 
                                       c, 
                                       comp))->second}
        ); 
        intervals[c] = 0;

        // print();
    }

    coord get_duplicate_dist() const {
        coord n = 0;
        // print_intervals();
        auto it = intervals.begin(); 
        auto begin = intervals.end();
        while (it != intervals.end()) {
            if (it->second > 1) {
                if (begin == intervals.end()) begin = it;
            }
            else {
                if (begin != intervals.end()) n += it->first - begin->first;
                begin = intervals.end();
            }

            ++it;
        }
        return n;
    }

    void print() const { print_intervals(); }

private:
    std::map<coord, int> intervals;

    class CompareClass {
    public:
        bool operator() (const std::pair<coord, int> &a, const coord &b) const {
            return a.first < b;
        }
        bool operator() (const coord &a, const std::pair<coord, int> &b) const {
            return a < b.first;
        }
    } comp;

    void print_intervals() const {
        for (auto &p: intervals) {
            std::cout << p.first << ": " << p.second << std::endl;
        }
        std::cout << std::endl;
    }
};

int main() {
    std::ifstream is("input.txt");
    std::vector<Line> lines, h_lines, v_lines;
    while (is) {
        Line l(is);
        if (l.is_horizontal()) {
            lines.push_back(l);
            h_lines.push_back(l);
        }
        else if (l.is_vertical()) {
            lines.push_back(l);
            v_lines.push_back(l);
        }
    }

    std::unordered_map<coord, NumberLine> h, v;

    for (const Line &l: lines) {
        // std::cout << l.is_horizontal() << l.is_vertical() << std::endl;
        if (l.is_horizontal()) {
            h[l.get_coord()].add_interval(l.get_interval());
        }
        else if (l.is_vertical()) {
            v[l.get_coord()].add_interval(l.get_interval());
        }
    }

    for (const Line &h_l: h_lines) {
        for (const Line &v_l: v_lines) {
            Point p = lines_cross(h_l, v_l);
            if (p != Point()) {
                // std::cout << p << std::endl;
                h[p.y()].add_interval({p.x(), next_coord(p.x())});
                v[p.x()].zero_coord(p.y());
            }
        }
    }

    // std::cout << v[3].get_duplicate_dist() << std::endl;
    // v[3].print();
    // h[7].print();

    int total = 0;
    for (const auto &p: h) total += p.second.get_duplicate_dist();
    for (const auto &p: v) total += p.second.get_duplicate_dist();

    std::cout << total << std::endl;


}