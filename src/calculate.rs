use pyo3::{
    prelude::*,
    types::{PyList, PyModule},
};

fn calc_hand_python(board_array: &[&str], players_cards_array: &[&str]) -> PyResult<Vec<f64>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let activators = PyModule::import(py, "holdem_calc")?;

    let board: Py<PyList> = PyList::new(py, board_array).into();
    let players_cards: Py<PyList> = PyList::new(py, players_cards_array).into();

    let result: Vec<f64> = activators
        .call1(
            "calculate",
            (board, false, 1, activators.None(), players_cards, false),
        )?
        .extract()?;
    Ok(result)
}

pub fn calc_hand(board_array: &[&str], players_cards_array: &[&str]) -> Vec<f64> {
    calc_hand_python(board_array, players_cards_array).unwrap()
}
