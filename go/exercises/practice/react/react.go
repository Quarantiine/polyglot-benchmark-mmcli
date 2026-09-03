package react

import (
	"container/heap"
)

// reactiveCell defines the internal interface for cells participating in dependency tracking and propagation.
type reactiveCell interface {
	Cell
	getLevel() int
	addConsumer(c *computeCell)
}

// reactor manages linked cells.
type reactor struct{}

// inputCell holds a settable value and propagates changes downstream.
type inputCell struct {
	r         *reactor
	val       int
	consumers []*computeCell
}

// callbackEntry stores a registered callback and its unique identifier for cancellation.
type callbackEntry struct {
	id int
	fn func(int)
}

// computeCell computes its value dynamically from upstream dependencies and triggers callbacks on stable changes.
type computeCell struct {
	r              *reactor
	val            int
	stableVal      int
	level          int
	computeFn      func() int
	consumers      []*computeCell
	callbacks      []callbackEntry
	nextCallbackID int
	inQueue        bool
}

// canceler removes a registered callback from its corresponding compute cell.
type canceler struct {
	cell *computeCell
	id   int
}

// Value returns the current value of the input cell.
func (c *inputCell) Value() int {
	return c.val
}

// getLevel returns the topological level (0 for inputs).
func (c *inputCell) getLevel() int {
	return 0
}

// addConsumer registers a downstream compute cell that depends on this input cell.
func (c *inputCell) addConsumer(consumer *computeCell) {
	c.consumers = append(c.consumers, consumer)
}

// SetValue sets the value of the input cell and triggers topological update propagation.
func (c *inputCell) SetValue(value int) {
	if c.val == value {
		return
	}
	c.val = value
	c.r.propagate(c.consumers)
}

// Value returns the current value of the compute cell.
func (c *computeCell) Value() int {
	return c.val
}

// getLevel returns the topological level of the compute cell.
func (c *computeCell) getLevel() int {
	return c.level
}

// addConsumer registers a downstream compute cell that depends on this compute cell.
func (c *computeCell) addConsumer(consumer *computeCell) {
	c.consumers = append(c.consumers, consumer)
}

// AddCallback registers a callback to be called when the cell's stable value changes.
func (c *computeCell) AddCallback(callback func(int)) Canceler {
	id := c.nextCallbackID
	c.nextCallbackID++
	c.callbacks = append(c.callbacks, callbackEntry{id: id, fn: callback})
	return &canceler{
		cell: c,
		id:   id,
	}
}

// removeCallback removes a callback by its identifier.
func (c *computeCell) removeCallback(id int) {
	for i, cb := range c.callbacks {
		if cb.id == id {
			c.callbacks = append(c.callbacks[:i], c.callbacks[i+1:]...)
			return
		}
	}
}

// hasCallback checks if a callback with the given identifier is currently active.
func (c *computeCell) hasCallback(id int) bool {
	for _, cb := range c.callbacks {
		if cb.id == id {
			return true
		}
	}
	return false
}

// Cancel removes the registered callback from the compute cell.
func (c *canceler) Cancel() {
	if c.cell == nil {
		return
	}
	c.cell.removeCallback(c.id)
	c.cell = nil
}

// cellHeap implements container/heap.Interface for compute cells ordered by topological level.
type cellHeap []*computeCell

func (h cellHeap) Len() int           { return len(h) }
func (h cellHeap) Less(i, j int) bool { return h[i].level < h[j].level }
func (h cellHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *cellHeap) Push(x any)        { *h = append(*h, x.(*computeCell)) }
func (h *cellHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

// propagate computes updated values in topological order and fires callbacks once a stable state is reached.
func (r *reactor) propagate(initialConsumers []*computeCell) {
	if len(initialConsumers) == 0 {
		return
	}

	h := make(cellHeap, 0, len(initialConsumers))
	for _, c := range initialConsumers {
		if !c.inQueue {
			c.inQueue = true
			heap.Push(&h, c)
		}
	}

	var changedComputeCells []*computeCell

	for h.Len() > 0 {
		c := heap.Pop(&h).(*computeCell)
		c.inQueue = false

		newVal := c.computeFn()
		if newVal != c.val {
			c.val = newVal
			changedComputeCells = append(changedComputeCells, c)
			for _, consumer := range c.consumers {
				if !consumer.inQueue {
					consumer.inQueue = true
					heap.Push(&h, consumer)
				}
			}
		}
	}

	for _, c := range changedComputeCells {
		if c.val != c.stableVal {
			c.stableVal = c.val
			cbs := make([]callbackEntry, len(c.callbacks))
			copy(cbs, c.callbacks)
			for _, cb := range cbs {
				if c.hasCallback(cb.id) {
					cb.fn(c.val)
				}
			}
		}
	}
}

// New creates and returns a new Reactor instance.
func New() Reactor {
	return &reactor{}
}

// CreateInput creates an input cell linked into the reactor with the given initial value.
func (r *reactor) CreateInput(initial int) InputCell {
	return &inputCell{
		r:   r,
		val: initial,
	}
}

// CreateCompute1 creates a compute cell which computes its value based on one other cell.
func (r *reactor) CreateCompute1(dep Cell, compute func(int) int) ComputeCell {
	rc := dep.(reactiveCell)
	level := rc.getLevel() + 1
	initialVal := compute(dep.Value())

	c := &computeCell{
		r:         r,
		val:       initialVal,
		stableVal: initialVal,
		level:     level,
		computeFn: func() int {
			return compute(dep.Value())
		},
	}
	rc.addConsumer(c)
	return c
}

// CreateCompute2 creates a compute cell which computes its value based on two other cells.
func (r *reactor) CreateCompute2(dep1, dep2 Cell, compute func(int, int) int) ComputeCell {
	rc1 := dep1.(reactiveCell)
	rc2 := dep2.(reactiveCell)
	level := rc1.getLevel()
	if rc2.getLevel() > level {
		level = rc2.getLevel()
	}
	level++

	initialVal := compute(dep1.Value(), dep2.Value())

	c := &computeCell{
		r:         r,
		val:       initialVal,
		stableVal: initialVal,
		level:     level,
		computeFn: func() int {
			return compute(dep1.Value(), dep2.Value())
		},
	}
	rc1.addConsumer(c)
	rc2.addConsumer(c)
	return c
}
