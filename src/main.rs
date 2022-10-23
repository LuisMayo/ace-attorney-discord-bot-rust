use pyo3::{prelude::*};
use pyo3::types::IntoPyDict;

mod comment;
mod Comment;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let engine = py.import("objection_engine")?;
        // let locals = [("objection_engine",engine)].into_py_dict(py);
        let kwargs = [("text_content", "hola hola majisimos"),("user_name", "user1")].into_py_dict(py);
        let comment1 = engine.getattr("comment")?.call_method0("Comment")?;
        let comment2 = engine.getattr("comment")?.call_method("Comment", ("", ), Some(kwargs))?;
        let comments = py.eval("[]", None, None)?;
        comments.call_method1("append", (comment1,))?;
        comments.call_method1("append", (comment2,))?;
        // let comments = py.eval("[objection_engine.comment.Comment()]", None, Some(locals)).unwrap();//.to_object(py);
        engine.getattr("renderer")?.call_method1("render_comment_list", (comments,))?;
        Ok(())
    })
}
