import java.lang.reflect.Array;

class SimpleLinkedList<T> {
    private Node<T> head;
    private int size;

    private static class Node<T> {
        T data;
        Node<T> next;

        Node(T data) {
            this.data = data;
        }
    }

    SimpleLinkedList() {
        this.head = null;
        this.size = 0;
    }

    SimpleLinkedList(T[] values) {
        this.head = null;
        this.size = 0;
        for (T value : values) {
            push(value);
        }
    }

    void push(T value) {
        Node<T> newNode = new Node<>(value);
        newNode.next = head;
        head = newNode;
        size++;
    }

    T pop() {
        if (head == null) {
            throw new java.util.NoSuchElementException("List is empty");
        }
        T data = head.data;
        head = head.next;
        size--;
        return data;
    }

    void reverse() {
        Node<T> prev = null;
        Node<T> current = head;
        Node<T> next = null;
        while (current != null) {
            next = current.next;
            current.next = prev;
            prev = current;
            current = next;
        }
        head = prev;
    }

    @SuppressWarnings("unchecked")
    T[] asArray(Class<T> clazz) {
        T[] array = (T[]) Array.newInstance(clazz, size);
        Node<T> current = head;
        for (int i = 0; i < size; i++) {
            array[i] = current.data;
            current = current.next;
        }
        return array;
    }

    int size() {
        return size;
    }
}
