
class List {
    __InternalList __list;


    List();


    void add(el) {
        __LIST_ADD(__list, el);
    }


    void clear() {
        __LIST_CLEAR(__list);
    }


    void removeLast() {
        // Returns the removed element.
        return __LIST_REMOVELAST(__list);
    }


    String toString() {
        return __LIST_TOSTRING(__list);
    }
}

