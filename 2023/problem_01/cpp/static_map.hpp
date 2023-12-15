// static_map.hpp

#pragma once

#include <cinttypes>
#include <string>
#include <unordered_map>

///
/// Structures
///

template <typename K, typename V, size_t N>
class static_map {
  private:
    // key-value store as contiguous arrays
    size_t m_size = N;
    K m_keys[N];
    V m_values[N];
    // search for key index (used to return values)
    size_t _get_key_index(K key) const {
        size_t i;
        for (i=0; i<N; ++i) {
            if (m_keys[i] == key) return i;
            i++;
        }
        throw "Key not found.";
    }
    V _get_value(K key) const {
        size_t i = _get_key_index(key);
        return m_values[i];
    }
  public:
    // constructor: two arrays
    static_map(K keys[], V values[]) {
        size_t k_size = sizeof(*keys) / sizeof(K);
        size_t v_size = sizeof(*values) / sizeof(V);
        if (k_size != v_size || k_size != N || v_size != N) throw "Array size mismatch.";
        m_keys   = keys;
        m_values = values;
    }
    // constructor: vector of pair
    static_map(std::initializer_list<std::pair<K,V>> kv) {
        if (kv.size() != N) throw "Array size mismatch.";
		size_t i = 0 ;
		for (auto element : kv) {
			m_keys[i] = element.first;
            m_values[i] = element.second;
			++i;
		}
	}
    // default destructor
    ~static_map() {}
    // check if key exists
    bool exists(K key) {
        size_t i = _get_key_index(key);
        return (!i) ? false : true;
    }
    // get array size
    size_t size() {
        return m_size;
    }
    // get key index
    size_t index(K key) {
        size_t i = _get_key_index(key);
        return (!i) ? -1 : i;
    }
    // get value by index
    V get(size_t i) const {
        return m_values[i];
    }
    // get value by key
    V get(K key) const {
        return _get_value(key);
    }
    // indexing
    V& operator[](size_t i) const {
        if (i >= N || i < 0) throw "Index out of range.";
        return m_values[i];
    }
};

///
/// Functions
///

uint32_t wordtoi1(std::string integer) {
    std::unordered_map<std::string, uint32_t> lut = {
        {"one",   1},
        {"two",   2},
        {"three", 3},
        {"four",  4},
        {"five",  5},
        {"six",   6},
        {"seven", 7},
        {"eight", 8},
        {"nine",  9}
    };
    return lut[integer];
}

uint32_t wordtoi2(std::string integer) {
    static_map<std::string, uint32_t, 9> lut = {
        {"one",   1},
        {"two",   2},
        {"three", 3},
        {"four",  4},
        {"five",  5},
        {"six",   6},
        {"seven", 7},
        {"eight", 8},
        {"nine",  9}
    };
    return lut.get(integer);
}

uint32_t wordtoi2(size_t index) {
    static_map<std::string, uint32_t, 9> lut = {
        {"one",   1},
        {"two",   2},
        {"three", 3},
        {"four",  4},
        {"five",  5},
        {"six",   6},
        {"seven", 7},
        {"eight", 8},
        {"nine",  9}
    };
    return lut.get(index-1);
}
