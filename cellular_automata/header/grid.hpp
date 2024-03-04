#ifndef GRID
#define GRID

#include <utility>
#include <iostream>
#include <vector>

class Index_Out_Of_Range : public std::exception {
public:
    const char* what() {
        return  "Index is out of range\n";
    }

};

template<typename T>
class Grid {
private:

    std::vector<T*> grid;
    int32_t grid_size     = -1;
    int32_t x_len, y_len  = -1;


    void copy(const Grid<T> &obj);
    void move(Grid<T> &obj);
    void clean();

    static int32_t default_len, default_x, default_y;

public:

    Grid(int32_t x, int32_t y);
    Grid();

    // Copy and move semantics
    Grid(const Grid<T>& obj);
    Grid(Grid<T>&& obj);

    Grid& operator=(const Grid<T>& obj);
    Grid& operator=(Grid<T>&& obj);

    T& operator[](std::pair<int32_t, int32_t> coords);
    T& operator[](int32_t index);

    const T& operator[](std::pair<int32_t, int32_t> coords) const;
    const T& operator[](int32_t index) const;

    friend std::ostream& operator<<(std::ostream& os, const Grid<T> &obj) {

        int32_t cnt = 0;
        for(auto &it : obj.grid) {
            os << *it << " | ";
            cnt++;
            if(cnt % obj.width() == 0) { 
                cnt = 0;
                os << std::endl;
            }
        }

        return os;
    }

    // add const []

    int32_t size() const;
    int32_t width() const;
    int32_t heigth() const;

    // For iterator
    
    auto begin() { return grid.begin();}
    auto end() {return grid.end();}

    auto begin() const { return grid.cbegin();}
    auto end() const {return grid.cend();}

    ~Grid();

};

template<typename T>
int32_t Grid<T>::default_len = 100;

template<typename T>
int32_t Grid<T>::default_x = 100;

template<typename T>
int32_t Grid<T>::default_y = 100;


template <typename T>
Grid<T>::Grid(int32_t x, int32_t y) : x_len(x), y_len(y), grid_size(x*y) {
    grid = std::vector<T*>(grid_size);

    for(auto &it : grid)
        it = new T;

}

template <typename T>
Grid<T>::Grid() : x_len(default_x), y_len(default_y), grid_size(default_len) {
    grid = std::vector<T*>(grid_size);

    for(auto &it : grid)
        it = new T;
}

// copy and move semantics

// Helper functions

template<typename T>
auto Grid<T>::copy(const Grid<T> &obj) -> void {
    grid_size = obj.grid_size;
    x_len = obj.x_len;
    y_len = obj.y_len;

    grid = std::vector<T*>(grid_size);

    for(int i = 0; i < grid_size; i++) {
        grid[i] = new T(obj[i]);
    }


}

template<typename T>
auto Grid<T>::move(Grid<T> &obj) -> void {

    this->grid_size = obj.grid_size;
    this->x_len = obj.x_len;
    this->y_len = obj.y_len;

    this->grid = std::move(obj.grid);

    // all others relese
    obj.grid_size = obj.x_len = obj.y_len = -1;

}

template<typename T>
auto Grid<T>::clean() -> void {
    for(auto &it : grid)
        delete it;

    grid_size = -1;
    x_len = -1;
    y_len = -1;
    
}

template <typename T>
Grid<T>::Grid(const Grid<T> &obj) {
    clean();
    copy(obj);
}

template <typename T>
Grid<T>::Grid(Grid<T> &&obj) {
    move(obj);
}

template <typename T>
auto Grid<T>::operator=(const Grid<T> &obj) -> Grid<T>&{
    if(this != &obj) {
        clean();
        copy(obj);
    }
    return *this;
}

template <typename T>
auto Grid<T>::operator=(Grid<T> &&obj) -> Grid<T>& {
    if(this != &obj) {
        clean();
        move(obj);
    }
    return *this;
}

template <typename T>
auto Grid<T>::operator[](std::pair<int32_t, int32_t> coords) -> T& {
    int32_t x = coords.first;
    int32_t y = coords.second;

    // Error: handled
    if(x < 0 or x > x_len or y < 0 or y > y_len) throw Index_Out_Of_Range();
    
    return *(grid[y*x_len + x]);
}

template <typename T>
auto Grid<T>::operator[](int32_t index) -> T& {
    // Error: handled
    if(index < 0 or index > grid_size) throw Index_Out_Of_Range();
    return *(grid[index]);
}

template<typename T>
auto Grid<T>::operator[](std::pair<int32_t, int32_t> coords) const -> const T& {
    int32_t x = coords.first;
    int32_t y = coords.second;

    // Error: handled
    if(x < 0 or x > x_len or y < 0 or y > y_len) throw Index_Out_Of_Range();
    
    return *(grid[y*x_len + x]);

}

template<typename T>
auto Grid<T>::operator[](int32_t index) const -> const T& {
    // Error: handled
    if(index < 0 or index > grid_size) throw Index_Out_Of_Range();
    return *(grid[index]);
}


template<typename T>
auto Grid<T>::size() const -> int32_t {
    return this->grid_size;
}

template<typename T>
auto Grid<T>::width() const -> int32_t {
    return this->x_len;
}

template<typename T>
auto Grid<T>::heigth() const -> int32_t {
    return this->y_len;
}


template <typename T>
Grid<T>::~Grid() {
    clean();
}


#endif
