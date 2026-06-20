/**
 * Reactive Programming System.
 * 
 * This module implements a spreadsheet-like reactive system using a Directed Acyclic Graph (DAG)
 * of cells. It consists of:
 * - `InputCell`: Value sources that can be manually updated.
 * - `ComputeCell`: Derived cells whose values depend on other cells and are updated automatically.
 * - `CallbackCell`: Observers registered on compute cells that record state changes.
 * 
 * Architectural Highlights:
 * 1. Topological Sorting: Updates propagate from modified InputCells to dependent ComputeCells in
 *    topological order. This guarantees each cell is recalculated exactly once and always has the latest
 *    values from its inputs, preventing glitching or inconsistent intermediate states.
 * 2. Stable State Callbacks: Callbacks are only invoked after the entire network has settled into a
 *    stable state. A callback is triggered only if the cell's final value differs from its value at the
 *    start of the transaction (setValue invocation).
 */

export class InputCell {
  /**
   * Creates a new input cell holding a primitive value.
   * @param {*} value - Initial value of the cell.
   */
  constructor(value) {
    this.value = value;
    /** @type {Set<ComputeCell>} Direct descendants in the reactive graph */
    this.dependents = new Set();
  }

  /**
   * Sets a new value for the input cell and triggers propagation through the dependency graph.
   * Updates only run if the new value is different from the current value (strict equality).
   * @param {*} value - The new value to assign.
   */
  setValue(value) {
    if (this.value === value) return;

    // 1. Traverse and discover all transitively dependent ComputeCells.
    // We must capture their values before any recomputations occur to accurately
    // determine if their final values in the new stable state have changed.
    const dependentComputeCells = [];
    const visited = new Set();
    
    const findComputeCells = (cell) => {
      for (const dep of cell.dependents) {
        if (!visited.has(dep)) {
          visited.add(dep);
          dependentComputeCells.push(dep);
          findComputeCells(dep);
        }
      }
    };
    findComputeCells(this);

    // Snapshot original stable values of the dependent compute cells.
    const originalValues = new Map();
    for (const cell of dependentComputeCells) {
      originalValues.set(cell, cell.value);
    }

    // Set the new value on this input cell.
    this.value = value;

    // 2. Perform a topological sort of all dependent compute cells.
    // This ensures that for any cell C depending on cells A and B, C is recomputed
    // only after both A and B are fully recomputed and stable.
    const topoOrder = [];
    const topoVisited = new Set();
    
    const topoVisit = (cell) => {
      if (topoVisited.has(cell)) return;
      topoVisited.add(cell);
      for (const dep of cell.dependents) {
        if (visited.has(dep)) {
          topoVisit(dep);
        }
      }
      topoOrder.push(cell);
    };

    for (const cell of dependentComputeCells) {
      topoVisit(cell);
    }
    // Reversing the post-order depth-first traversal yields a valid topological sort.
    topoOrder.reverse();

    // 3. Recompute each ComputeCell in topological order.
    for (const cell of topoOrder) {
      cell.value = cell.calculateValue();
    }

    // 4. Trigger callbacks for those compute cells whose values changed
    // from their original stable value to their final stable value.
    for (const cell of topoOrder) {
      const origVal = originalValues.get(cell);
      if (origVal !== cell.value) {
        // Create a shallow copy of the callback list to avoid mutation issues
        // if callbacks register or deregister other callbacks during invocation.
        for (const cb of Array.from(cell.callbacks)) {
          const val = cb.fn(cell);
          cb.values.push(val);
        }
      }
    }
  }
}

export class ComputeCell {
  /**
   * Creates a computed cell that automatically updates its value based on dependencies.
   * @param {Array<InputCell|ComputeCell>} inputCells - Upstream dependency cells.
   * @param {function(Array<InputCell|ComputeCell>): *} fn - Computation function.
   */
  constructor(inputCells, fn) {
    this.inputs = inputCells;
    this.fn = fn;
    /** @type {Set<ComputeCell>} Direct descendants in the reactive graph */
    this.dependents = new Set();
    /** @type {Set<CallbackCell>} Registered change notification observers */
    this.callbacks = new Set();
    
    // Register this cell as a dependent of all its input dependency cells.
    for (const input of this.inputs) {
      input.dependents.add(this);
    }
    
    // Compute the initial value.
    this.value = this.calculateValue();
  }

  /**
   * Computes the current value of the cell using the user-provided function.
   * @returns {*} Computed value.
   */
  calculateValue() {
    return this.fn(this.inputs);
  }

  /**
   * Registers a change notification callback cell.
   * @param {CallbackCell} cb - Callback cell to add.
   */
  addCallback(cb) {
    this.callbacks.add(cb);
  }

  /**
   * Removes a registered change notification callback cell.
   * @param {CallbackCell} cb - Callback cell to remove.
   */
  removeCallback(cb) {
    this.callbacks.delete(cb);
  }
}

export class CallbackCell {
  /**
   * Creates an observer that records state change notifications of a ComputeCell.
   * @param {function(ComputeCell): *} fn - Function to run on change.
   */
  constructor(fn) {
    this.fn = fn;
    /** @type {Array<*>} List of recorded return values from the callback function */
    this.values = [];
  }
}
