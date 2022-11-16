use pyo3::{Python};
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;

use crate::job_model::JobMsg;

use super::comment::Comment;
pub fn init_python() {
    let (s, r): (Sender<JobMsg>, Receiver<JobMsg>) = unbounded();
    let thread_s = s.clone();
    let thread_r = r.clone();
    thread::spawn(move || {
        Python::with_gil(|py| {
            let engine = py.import("objection_engine").unwrap();
            // let locals = [("objection_engine",engine)].into_py_dict(py);
            // let pyArr = list.iter().map(|item| item.to_comment(&py, &engine));
            loop {
                let job = thread_r.recv().unwrap();
                let pyarr = py.eval("[]", None, None).unwrap();
                for item in job..iter() {
                    let pycomment = item.to_comment(&py, &engine);
                    pyarr.call_method1("append", (pycomment,)).unwrap();
                }
                // let comments = py.eval("[objection_engine.comment.Comment()]", None, Some(locals)).unwrap();//.to_object(py);
                engine.getattr("renderer").unwrap().call_method1("render_comment_list", (pyarr, unique_id)).unwrap();
                // Ok(())
            }
        });
    });
}
pub fn render_comment_list(list: &Vec<Comment>, unique_id: u64) {
    
    return;
}