
class List {
    __InternalList __list;


    List();


    void add(el) {
        __list = __LIST_ADD(__list, el);
    }


    void clear() {
        __list = __LIST_CLEAR(__list);
    }


    String toString() {
        return __LIST_TOSTRING(__list);
    }
}

