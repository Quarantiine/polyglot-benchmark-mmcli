/// `InputCellId` is a unique identifier for an input cell.
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct ComputeCell<T> {
    dependencies: Vec<CellId>,
    compute_func: Box<dyn Fn(&[T]) -> T>,
    value: T,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T)>>,
    next_callback_id: usize,
}

pub struct Reactor<T> {
    inputs: HashMap<InputCellId, T>,
    computes: HashMap<ComputeCellId, ComputeCell<T>>,
    next_input_id: usize,
    next_compute_id: usize,
}

impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            computes: HashMap::new(),
            next_input_id: 0,
            next_compute_id: 0,
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.next_input_id);
        self.next_input_id += 1;
        self.inputs.insert(id, initial);
        id
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for &dep in dependencies {
            if !self.cell_exists(dep) {
                return Err(dep);
            }
        }

        let id = ComputeCellId(self.next_compute_id);
        self.next_compute_id += 1;

        let value = compute_func(&self.get_dependency_values(dependencies));
        self.computes.insert(
            id,
            ComputeCell {
                dependencies: dependencies.to_vec(),
                compute_func: Box::new(compute_func),
                value,
                callbacks: HashMap::new(),
                next_callback_id: 0,
            },
        );
        Ok(id)
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.inputs.get(&id).copied(),
            CellId::Compute(id) => self.computes.get(&id).map(|c| c.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if !self.inputs.contains_key(&id) {
            return false;
        }

        if self.inputs[&id] == new_value {
            return true;
        }

        self.inputs.insert(id, new_value);
        self.update_computes();
        true
    }

    pub fn add_callback<F: FnMut(T) + 'static>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let cell = self.computes.get_mut(&id)?;
        let cb_id = CallbackId(cell.next_callback_id);
        cell.next_callback_id += 1;
        cell.callbacks.insert(cb_id, Box::new(callback));
        Some(cb_id)
    }

    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cell = self.computes
            .get_mut(&cell_id)
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        let _ = cell.callbacks
            .remove(&callback_id)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        Ok(())
    }

    fn cell_exists(&self, id: CellId) -> bool {
        match id {
            CellId::Input(id) => self.inputs.contains_key(&id),
            CellId::Compute(id) => self.computes.contains_key(&id),
        }
    }

    fn get_dependency_values(&self, dependencies: &[CellId]) -> Vec<T> {
        dependencies.iter().map(|&id| self.value(id).unwrap()).collect()
    }

    fn update_computes(&mut self) {
        let old_values: HashMap<ComputeCellId, T> = self.computes.iter().map(|(&id, cell)| (id, cell.value)).collect();

        let mut changed = true;
        while changed {
            changed = false;
            let compute_ids: Vec<ComputeCellId> = self.computes.keys().cloned().collect();
            for id in compute_ids {
                let new_value = {
                    let cell = &self.computes[&id];
                    (cell.compute_func)(&self.get_dependency_values(&cell.dependencies))
                };
                if new_value != self.computes[&id].value {
                    self.computes.get_mut(&id).unwrap().value = new_value;
                    changed = true;
                }
            }
        }

        let mut changed_ids = Vec::new();
        for (&id, cell) in &self.computes {
            if cell.value != *old_values.get(&id).unwrap() {
                changed_ids.push(id);
            }
        }

        for id in changed_ids {
            let cell = self.computes.get_mut(&id).unwrap();
            for cb in cell.callbacks.values_mut() {
                cb(cell.value);
            }
        }
    }
}