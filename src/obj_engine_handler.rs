use pyo3::{Python};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;

use crate::job_model::{JobMsg};

pub fn init_python() -> MySender<JobMsg> {
    let (s, thread_r): (SyncSender<JobMsg>, Receiver<JobMsg>) = mpsc::sync_channel(500);
    thread::spawn(move || {
        Python::with_gil(move |py| {
            let engine = py.import("objection_engine").unwrap();
            // let locals = [("objection_engine",engine)].into_py_dict(py);
            // let pyArr = list.iter().map(|item| item.to_comment(&py, &engine));
            loop {
                let job = thread_r.recv().unwrap();
                let pyarr = py.eval("[]", None, None).unwrap();
                for item in job.job_model.msgs.iter() {
                    let pycomment = item.to_comment(&py, &engine);
                    pyarr.call_method1("append", (pycomment,)).unwrap();
                }
                // let comments = py.eval("[objection_engine.comment.Comment()]", None, Some(locals)).unwrap();//.to_object(py);
                // TODO luis.mayo check this
                // let unique_id = Python::In
                engine.getattr("renderer").unwrap().call_method1("render_comment_list", (pyarr, job.job_model.discord_msg.id.0.to_string() + ".mp4")).unwrap();
                // Ok(())
            }
        });
    });
    return MySender(s);
}

pub struct MySender<T>(pub SyncSender<T>);

impl serenity::prelude::TypeMapKey for MySender<JobMsg> {
    type Value = MySender<JobMsg>;
}
