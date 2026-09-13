import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.IdentityHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Queue;
import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Function;

public class React {

    public static class Cell<T> {
        protected T value;
        final List<ComputeCell<?>> consumers = new ArrayList<>();
        int level = 0;

        public T getValue() {
            return value;
        }

        int getLevel() {
            return level;
        }
    }

    public static class InputCell<T> extends Cell<T> {
        public InputCell(T initialValue) {
            this.value = initialValue;
            this.level = 0;
        }

        public void setValue(T newValue) {
            if (Objects.equals(this.value, newValue)) {
                return;
            }
            this.value = newValue;
            propagate();
        }

        private void propagate() {
            // Collect all reachable compute cells downstream
            Set<ComputeCell<?>> reachable = new LinkedHashSet<>();
            Queue<Cell<?>> queue = new ArrayDeque<>();
            queue.add(this);

            while (!queue.isEmpty()) {
                Cell<?> current = queue.poll();
                for (ComputeCell<?> consumer : current.consumers) {
                    if (reachable.add(consumer)) {
                        queue.add(consumer);
                    }
                }
            }

            if (reachable.isEmpty()) {
                return;
            }

            // Capture stable state values prior to this update cycle
            Map<ComputeCell<?>, Object> oldValues = new IdentityHashMap<>();
            for (ComputeCell<?> cell : reachable) {
                oldValues.put(cell, cell.getValue());
            }

            // Topological sort by DAG depth level to guarantee dependencies resolve first
            List<ComputeCell<?>> sortedCells = new ArrayList<>(reachable);
            sortedCells.sort(Comparator.comparingInt(Cell::getLevel));

            // Recompute values in topological order to bring all cells into the new stable state
            for (ComputeCell<?> cell : sortedCells) {
                cell.recompute();
            }

            // Fire callbacks only for cells whose value changed between stable states
            for (ComputeCell<?> cell : sortedCells) {
                Object oldValue = oldValues.get(cell);
                Object newValue = cell.getValue();
                if (!Objects.equals(oldValue, newValue)) {
                    cell.fireCallbacks();
                }
            }
        }
    }

    public static class ComputeCell<T> extends Cell<T> {
        private final Function<List<T>, T> function;
        private final List<Cell<T>> cells;
        private final List<Consumer<T>> callbacks = new ArrayList<>();

        public ComputeCell(Function<List<T>, T> function, List<Cell<T>> cells) {
            this.function = function;
            this.cells = new ArrayList<>(cells);

            int maxLevel = 0;
            Set<Cell<T>> uniqueCells = new LinkedHashSet<>(cells);
            for (Cell<T> cell : uniqueCells) {
                maxLevel = Math.max(maxLevel, cell.getLevel());
                cell.consumers.add(this);
            }
            this.level = maxLevel + 1;
            this.value = computeValue();
        }

        void recompute() {
            this.value = computeValue();
        }

        private T computeValue() {
            List<T> values = new ArrayList<>(cells.size());
            for (Cell<T> cell : cells) {
                values.add(cell.getValue());
            }
            return function.apply(values);
        }

        public void addCallback(Consumer<T> callback) {
            callbacks.add(callback);
        }

        public void removeCallback(Consumer<T> callback) {
            callbacks.remove(callback);
        }

        void fireCallbacks() {
            List<Consumer<T>> snapshot = new ArrayList<>(callbacks);
            for (Consumer<T> callback : snapshot) {
                if (callbacks.contains(callback)) {
                    callback.accept(this.value);
                }
            }
        }
    }

    public static <T> InputCell<T> inputCell(T initialValue) {
        return new InputCell<>(initialValue);
    }

    public static <T> ComputeCell<T> computeCell(Function<List<T>, T> function, List<Cell<T>> cells) {
        return new ComputeCell<>(function, cells);
    }
}
