class InputCell:
    def __init__(self, initial_value):
        self._value = initial_value
        self.consumers = []

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, new_value):
        if self._value != new_value:
            self._value = new_value
            propagate(self)


class ComputeCell:
    def __init__(self, inputs, compute_function):
        self.inputs = inputs
        self.compute_function = compute_function
        self.consumers = []
        self.callbacks = []

        for input_cell in self.inputs:
            input_cell.consumers.append(self)

        self._value = self.compute_function([inp.value for inp in self.inputs])

    @property
    def value(self):
        return self._value

    def add_callback(self, callback):
        self.callbacks.append(callback)

    def remove_callback(self, callback):
        if callback in self.callbacks:
            self.callbacks.remove(callback)


def get_reachable_compute_cells(start_cell):
    reachable = set()
    queue = [start_cell]
    while queue:
        curr = queue.pop(0)
        for consumer in curr.consumers:
            if consumer not in reachable:
                reachable.add(consumer)
                queue.append(consumer)
    return reachable


def topological_sort(reachable_cells):
    in_degree = {cell: 0 for cell in reachable_cells}
    for cell in reachable_cells:
        for consumer in cell.consumers:
            if consumer in reachable_cells:
                in_degree[consumer] += 1

    queue = [cell for cell, deg in in_degree.items() if deg == 0]
    topo_order = []

    while queue:
        curr = queue.pop(0)
        topo_order.append(curr)
        for consumer in curr.consumers:
            if consumer in reachable_cells:
                in_degree[consumer] -= 1
                if in_degree[consumer] == 0:
                    queue.append(consumer)

    return topo_order


def propagate(start_cell):
    reachable = get_reachable_compute_cells(start_cell)
    if not reachable:
        return

    topo_order = topological_sort(reachable)

    # Record initial values
    initial_values = {cell: cell.value for cell in topo_order}

    # Update values in topological order
    for cell in topo_order:
        cell._value = cell.compute_function([inp.value for inp in cell.inputs])

    # Trigger callbacks for cells whose values changed
    for cell in topo_order:
        if cell.value != initial_values[cell]:
            for callback in list(cell.callbacks):
                callback(cell.value)
