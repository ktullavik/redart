
class List {
    __InternalList __list;


    List();


    E get first {
        return __LIST_GET_FIRST(__list);
    }


    E get last {
        return __LIST_GET_LAST(__list);
    }


    int get length {
        return __LIST_GET_LENGTH(__list);
    }


    // Adds value to the end of this list, extending the length by one. 
    void add(el) {
        __LIST_ADD(__list, el);
    }


    // Appends all objects of iterable to the end of this list.
    //
    // TODO
    // void addAll(Iterable<E> iterable) {
    void addAll(List iterable) {
        __LIST_ADDALL(__list, iterable);
    }


    // An unmodifiable Map view of this list. 
    // Map<int, E> asMap() {
    //     assert(false, "Not implemented: List.asMap()");
    // }


    // Returns a view of this list as a list of R instances.
    // override
    // List<R> cast<R>() {
    //     assert(false, "Not implemented: List.cast<R>()");
    // }


    // Removes all objects from this list; the length of the list becomes zero. 
    void clear() {
        __LIST_CLEAR(__list);
    }


    // Overwrites a range of elements with fillValue. 
    // void fillRange(int start, int end, [E? fillValue]) {
    //     assert(false, "Not implemented: List.fillRange()");
    // }


    // Creates an Iterable that iterates over a range of elements. 
    // Iterable<E> getRange(int start, int end) {
    //     assert(false, "Not implemented: List.getRange()");
    // }


    // The first index of element in this list. 
    // int indexOf(E element, [int start = 0]) {
    //     assert(false, "Not implemented: List.indexOf()");
    // }


    // The first index in the list that satisfies the provided test. 
    // int indexWhere(bool test(E element), [int start = 0]) {
    //     assert(false, "Not implemented: List.indexWhere()");
    // }


    // Inserts element at position index in this list. 
    void insert(int index, E element) {
        __LIST_INSERT(__list, index, element);
    }


    // Inserts all objects of iterable at position index in this list. 
    // void insertAll(int index, Iterable<E> iterable) {
    //     assert(false, "Not implemented: List.insertAll()");
    // }


    // The last index of element in this list. 
    // int lastIndexOf(E element, [int? start]) {
    //     assert(false, "Not implemented: List.lastIndexOf()");
    // }


    // The last index in the list that satisfies the provided test. 
    // int lastIndexWhere(bool test(E element), [int? start]) {
    //     assert(false, "Not implemented: List.lastIndexWhere()");
    // }


    // Removes the first occurrence of value from this list. 
    // bool remove(Object? value) {
    //     assert(false, "Not implemented: List.remove()");
    // }


    // Removes the object at position index from this list. 
    E removeAt(int index) {
        __LIST_REMOVERANGE(__list, index, index + 1);
    }

    // Removes and returns the last object in this list. 
    E removeLast() {
        return __LIST_REMOVELAST(__list);
    }


    // Removes a range of elements from the list. 
    void removeRange(int start, int end) {
        __LIST_REMOVERANGE(__list, start, end);
    }


    // Removes all objects from this list that satisfy test. 
    // void removeWhere(bool test(E element)) {
    //     assert(false, "Not implemented: List.removeWhere()");
    // }


    // Replaces a range of elements with the elements of replacements. 
    // void replaceRange(int start, int end, Iterable<E> replacements) {
    //     assert(false, "Not implemented: List.replaceRange()");
    // }


    // Removes all objects from this list that fail to satisfy test. 
    // void retainWhere(bool test(E element)) {
    //     assert(false, "Not implemented: List.retainWhere()");
    // }


    // Overwrites elements with the objects of iterable. 
    // void setAll(int index, Iterable<E> iterable) {
    //     assert(false, "Not implemented: List.setAll()");
    // }


    // Writes some elements of iterable into a range of this list. 
    // void setRange(int start, int end, Iterable<E> iterable, [int skipCount = 0]) {
    //     assert(false, "Not implemented: List.setRange()");
    // }


    // Shuffles the elements of this list randomly.
    // 
    // TODO
    // void shuffle([Random? random]) {
    void shuffle() {
        __LIST_SHUFFLE(__list);
    }


    // Sorts this list according to the order specified by the compare function. 
    // void sort([int compare(E a, E b)?]) {
    //     assert(false, "Not implemented: List.sort()");
    // }


    // Returns a new list containing the elements between start and end. 
    // List<E> sublist(int start, [int? end]) {
    //     assert(false, "Not implemented: List.sublist()");
    // }


    // Return a string representation of the list.
    String toString() {
        return __LIST_TOSTRING(__list);
    }


    // Creates a new lazy Iterable with all elements that have type T. 
    // Iterable<T> whereType<T>() {
    //     assert(false, "Not implemented: List.whereType<T>()");
    // }
}

