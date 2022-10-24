use pyo3::{Python};

use super::comment::Comment;
pub fn render_comment_list(list: &Vec<Comment>) {
    Python::with_gil(|py| {
        let engine = py.import("objection_engine").unwrap();
        // let locals = [("objection_engine",engine)].into_py_dict(py);
        // let pyArr = list.iter().map(|item| item.to_comment(&py, &engine));
        let pyarr = py.eval("[]", None, None).unwrap();
        for item in list.iter() {
            let pycomment = item.to_comment(&py, &engine);
            pyarr.call_method1("append", (pycomment,)).unwrap();
        }
        // let comments = py.eval("[objection_engine.comment.Comment()]", None, Some(locals)).unwrap();//.to_object(py);
        engine.getattr("renderer").unwrap().call_method1("render_comment_list", (pyarr,)).unwrap();
        // Ok(())
    });
    return;
}